#![allow(unused)]

pub fn solution()
{
    // Type is Render<Box<dyn FnMut(&mut UpdateEvent)>>
    let renderer = Render::new_box_mut("name1".to_string());
    // Type is Render<Box<dyn Fn(&mut UpdateEvent)>>
    let renderer = Render::new_box("name1".to_string());
    // Type is Render<fn(&mut UpdateEvent)>
    let renderer = Render::new_ptr("name2".to_string());
    // Type is Render< Unnameable closure type >
    // (Every closure has it's own unique type)
    let renderer = Render
    {
        name: "name3".to_string(),
        f_on_update: |e| { println!( "f_on_update" ) },
    };
}

pub struct UpdateEvent
{
    pub dt: f64,
}

//

pub struct Render<OnUpdate>
where
    OnUpdate: FnMut(&mut UpdateEvent),
{
    pub name: String,
    pub f_on_update: OnUpdate,
}

//

impl Render<Box<dyn FnMut(&mut UpdateEvent)>>
{
    pub fn new_box_mut(name: String) -> Self
    {
        // This will handle the most closures
        let f_on_update = Box::new(|e: &mut UpdateEvent| {});
        Render { name, f_on_update }
    }
}

impl Render<Box<dyn Fn(&mut UpdateEvent)>>
{
    pub fn new_box(name: String) -> Self
    {
        // This will handle closures that don't mutate themselves
        let f_on_update = Box::new(|e: &mut UpdateEvent| {});
        Render { name, f_on_update }
    }
}

impl Render<fn(&mut UpdateEvent)>
{
    pub fn new_ptr(name: String) -> Self
    {
        // Making this closure into a function pointer is possible because
        // it doesn't capture any environment
        let f_on_update = |e: &mut UpdateEvent| {};
        Render { name, f_on_update }
    }
}
