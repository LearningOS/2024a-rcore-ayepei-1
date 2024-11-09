use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{block_current_and_run_next, current_process, current_task};
use crate::timer::{add_timer, get_time_ms};
use alloc::sync::Arc;
use alloc:: vec;
/// sleep syscall
pub fn sys_sleep(ms: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_sleep",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let expire_ms = get_time_ms() + ms;
    let task = current_task().unwrap();
    add_timer(expire_ms, task);
    block_current_and_run_next();
    0
}
/// mutex create syscall
pub fn sys_mutex_create(blocking: bool) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mutex: Option<Arc<dyn Mutex>> = if !blocking {
        Some(Arc::new(MutexSpin::new()))
    } else {
        Some(Arc::new(MutexBlocking::new()))
    };
    let mut process_inner = process.inner_exclusive_access();
    if let Some(id) = process_inner
        .mutex_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.mutex_list[id] = mutex;
        id as isize
    } else {
        process_inner.mutex_list.push(mutex);
        process_inner.mutex_list.len() as isize - 1
    }
}
/// mutex lock syscall
pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_lock",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let process_inner = process.inner_exclusive_access();
    let deadlock_detect=process_inner.enable_deadlock_detect;
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    //死锁检测
    if deadlock_detect==true{
        task_inner.mutex_request[mutex_id]+=1;
        drop(task_inner);
        if mutex_deadlock_detect()==-1{
            let mut task_inner = task.inner_exclusive_access();
            task_inner.mutex_request[mutex_id]-=1;
            return -0xDEAD;
        }
        mutex.lock();
        let mut task_inner = task.inner_exclusive_access();
        task_inner.mutex_request[mutex_id]-=1;
        task_inner.mutex_allocation[mutex_id]+=1;
    }
    else {
        drop(task_inner);
        mutex.lock();
    }
    0
    // let process = current_process();
    // let process_inner = process.inner_exclusive_access();
    // let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    // drop(process_inner);
    // drop(process);
    // mutex.lock();
    // 0
}
/// mutex unlock syscall
pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_unlock",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    //原来的
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    if process_inner.enable_deadlock_detect{
        task_inner.allocation[mutex_id]-=1;
    }
    drop(process_inner);
    drop(process);
    mutex.unlock();
    0
}
/// semaphore create syscall
pub fn sys_semaphore_create(res_count: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .semaphore_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.semaphore_list[id] = Some(Arc::new(Semaphore::new(res_count)));
        id
    } else {
        process_inner
            .semaphore_list
            .push(Some(Arc::new(Semaphore::new(res_count))));
        process_inner.semaphore_list.len() - 1
    };
    id as isize
}
/// semaphore up syscall
pub fn sys_semaphore_up(sem_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_up",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    if process_inner.enable_deadlock_detect{
        task_inner.allocation[sem_id]-=1;
    }

 
    drop(process_inner);
    sem.up();
    0
}
/// semaphore down syscall
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_down",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
   
    let task = current_task().unwrap();
    let process = task.process.upgrade().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    let deadlock_detect=process_inner.enable_deadlock_detect;
    drop(process_inner);
    
    //死锁检测
    if deadlock_detect==true{
        task_inner.request[sem_id]+=1;
        drop(task_inner);
        if semaphore_deadlock_detect()==-1{
            let mut task_inner = task.inner_exclusive_access();
            task_inner.request[sem_id]-=1;
            return -0xDEAD;
        }
        sem.down();
        let mut task_inner = task.inner_exclusive_access();
        task_inner.request[sem_id]-=1;
        task_inner.allocation[sem_id]+=1;
    }
    else {
        drop(task_inner);
        sem.down();
    }
    
    
    0
}
/// condvar create syscall
pub fn sys_condvar_create() -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .condvar_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.condvar_list[id] = Some(Arc::new(Condvar::new()));
        id
    } else {
        process_inner
            .condvar_list
            .push(Some(Arc::new(Condvar::new())));
        process_inner.condvar_list.len() - 1
    };
    id as isize
}
/// condvar signal syscall
pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_signal",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    drop(process_inner);
    condvar.signal();
    0
}
/// condvar wait syscall
pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_wait",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    condvar.wait(mutex);
    0
}
/// enable deadlock detection syscall
///
/// YOUR JOB: Implement deadlock detection, but might not all in this syscall
pub fn sys_enable_deadlock_detect(enabled: usize) -> isize {
    trace!("kernel: sys_enable_deadlock_detect NOT IMPLEMENTED");
    if enabled!=1&&enabled!=0{
        return -1;
    }
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    if enabled==1{
        process_inner.enable_deadlock_detect=true;
    }
    else{
        process_inner.enable_deadlock_detect=false;
    }
    0
}


///semaphore_deadlock_detect
pub fn semaphore_deadlock_detect()->isize{
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let tasks=&process_inner.tasks;
    let semaphores = &process_inner.semaphore_list;
    //定义相关变量
    let thread_len=tasks.len();
    let semaphore_len=semaphores.len();
    let mut work=vec![0; semaphore_len];
    let mut finish=vec![false;thread_len];
    let mut change=true;
    let mut allocation=vec![vec![0; semaphore_len];thread_len];
    let mut request=vec![vec![0; semaphore_len];thread_len];
    let mut total = vec![0; semaphore_len];
    //赋值
    for (i, semaphore_option) in semaphores.iter().enumerate(){
        if let Some(semaphore)=semaphore_option{
            total[i]=semaphore.all_count;
        }
    }
    for (i, task_option) in tasks.iter().enumerate(){
        if let Some(task)=task_option{
            let task_inner=task.inner_exclusive_access();
            for j in 0..semaphore_len{
                request[i][j]=task_inner.request[j];
                allocation[i][j]=task_inner.allocation[j];
            }
        }
    }
    drop(process_inner);
    for j in 0..semaphore_len {
        let mut allocated = 0;
        for i in 0..thread_len {
            allocated += allocation[i][j];
        }
        work[j] = total[j]- allocated;
    }

    while change {
        change=false;
        for i in 0..thread_len{
            if finish[i]==false{
                let mut flag=0;
                for j in 0..semaphore_len{
                    if request[i][j]>work[j]{
                        flag=1;
                        break;
                    }
                }
                if flag==0{
                    for j in 0..semaphore_len{
                        work[j]+=allocation[i][j];
                    }
                    finish[i]=true;
                    change=true;
                }
            }
            
        }
        if change==false{
            break;
        }
    }
    for i in 0..thread_len{
        if finish[i]==false{
            return -1;
        }
    }
    
    0
}

///mutex_deadlock_detect
pub fn mutex_deadlock_detect()->isize{
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let tasks=&process_inner.tasks;
    let mutexs = &process_inner.mutex_list;
    //定义相关变量
    let thread_len=tasks.len();
    let mutex_len=mutexs.len();
    let mut work=vec![0; mutex_len];
    let mut finish=vec![false;thread_len];
    let mut change=true;
    let mut allocation=vec![vec![0; mutex_len];thread_len];
    let mut request=vec![vec![0; mutex_len];thread_len];

    //赋值
    for (i, task_option) in tasks.iter().enumerate(){
        if let Some(task)=task_option{
            let task_inner=task.inner_exclusive_access();
            for j in 0..mutex_len{
                request[i][j]=task_inner.mutex_request[j];
                allocation[i][j]=task_inner.mutex_allocation[j];
            }
        }
    }
    drop(process_inner);
    for j in 0..mutex_len {
        let mut allocated = 0;
        for i in 0..thread_len {
            allocated += allocation[i][j];
        }
        work[j] = 1- allocated;
    }

    while change {
        change=false;
        for i in 0..thread_len{
            if finish[i]==false{
                let mut flag=0;
                for j in 0..mutex_len{
                    if request[i][j]>work[j]{
                        flag=1;
                        break;
                    }
                }
                if flag==0{
                    for j in 0..mutex_len{
                        work[j]+=allocation[i][j];
                    }
                    finish[i]=true;
                    change=true;
                }
            }
            
        }
        if change==false{
            break;
        }
    }
    for i in 0..thread_len{
        if finish[i]==false{
            return -1;
        }
    }
    0
}