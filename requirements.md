

struct Renderer<DS: DataStructure, RT: RayTracer> {
    obj: Obj
    ds: DS,
    tracer: RT,
}

impl Renderer {
    fn new(obj: Obj) -> Self {
        DS::new(obj)
    }

    fn render(&self) -> Bmp 
}


Obj --> Datastructure
            ^
            |
rays -------|





