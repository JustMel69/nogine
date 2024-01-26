use std::{thread::{self, JoinHandle}, io::{Read, Seek}};

use thiserror::Error;

use crate::{graphics::texture::{TextureCfg, load_texture_data, TextureError, RawTexData, Texture}, Res, assert_expr, crash};

pub struct ResourceLoader {
    resources: Vec<ResourceRequest>,
}

impl ResourceLoader {
    #[must_use] pub const fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    #[must_use] pub fn add_image<T: ImgReader + 'static>(&mut self, src: T, cfg: TextureCfg) -> &mut Self {
        self.resources.push(ResourceRequest::Image { reader: Box::new(src), cfg });
        return self;
    }

    #[must_use] pub fn dispatch(self) -> ResourcePromise {
        let threads = self.resources.into_iter().map(|r| {
            let thread = thread::spawn(|| load_res(r));
            return Some(thread);
        }).collect();
        
        return ResourcePromise { threads };
    }
}

type ResourceResult = Res<Resource, ResourceError>;

pub struct ResourcePromise {
    threads: Vec<Option<JoinHandle<ResourceResult>>>,
}

impl ResourcePromise {
    /// Returns `true` if all resources have been loaded.
    pub fn all_done(&self) -> bool {
        let mut count = 0;
        for x in &self.threads {
            match x {
                Some(x) => if x.is_finished() { count += 1 },
                None => count += 1,
            }
        }
        return count == self.threads.len();
    }

    /// Pulls an image from the `ResourcePromise`. Will return `None` if:
    /// - The resource has been pulled before.
    /// - Blocking is disabled and the resource is not ready yet.
    pub fn pull_image(&mut self, index: usize, blocking: bool) -> Option<Res<Texture, ResourceError>> {
        let res = self.pull_res(index, blocking);
        
        return res.map(|x| x.map(|x| {
            match x {
                Resource::Image(x) => Texture::new(x.data, x.fmt, x.dims, x.cfg),
                #[allow(unreachable_patterns)] _ => crash!("Resource is not an image!")
            }
        }))
    }

    fn pull_res(&mut self, index: usize, blocking: bool) -> Option<ResourceResult> {
        assert_expr!(index < self.threads.len(), "Index {index} out of bounds! (Resource number is {})", self.threads.len());

        let thread = &mut self.threads[index];
        if thread.is_some() {
            let mut swap_thread = None;
            std::mem::swap(&mut swap_thread, thread);

            if blocking || swap_thread.as_ref().unwrap().is_finished() {
                let res = swap_thread.unwrap().join().unwrap();
                return Some(res);
            }
            
            std::mem::swap(&mut swap_thread, thread);
        }
        return None;
    }
}


pub trait ImgReader : Send + Seek + Read {}
impl<T: Seek + Send + Read> ImgReader for T {}

enum ResourceRequest {
    Image { reader: Box<dyn ImgReader>, cfg: TextureCfg }
}

enum Resource {
    Image(RawTexData)
}

fn load_res(res: ResourceRequest) -> ResourceResult {
    match res {
        ResourceRequest::Image { reader, cfg } => load_texture_data(reader, cfg).map(Resource::Image).map_err(ResourceError::from),
    }
}

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("{0}")]
    TexError(#[from] TextureError)
}