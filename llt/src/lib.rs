use common::project::Project;

// TEST FUNCTIONS
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

//
// PROJECT FUNCTIONS
//
#[unsafe(no_mangle)]
pub extern "C" fn project_new() -> *mut Project {
    let p: Box<Project> = Box::new(Project::new());
    return Box::into_raw(p)
}

#[unsafe(no_mangle)]
pub extern "C" fn project_free(pp: *mut Project) {
    if pp.is_null() { return }   
    unsafe { drop(Box::from_raw(pp)) }
}

// Заготовка для функций изменения структур
#[unsafe(no_mangle)]
pub extern "C" fn project_(pp: *mut Project) -> *mut Project {
    if pp.is_null() { return pp }   
    unsafe { 
        let p = Box::from_raw(pp);
        return Box::into_raw(p)
    }
}