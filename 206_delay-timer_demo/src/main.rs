use std::time::Duration;
use anyhow::Result;
use delay_timer::prelude::*;
use tokio::time;

fn main() -> Result<()> {
    let delay_timer = DelayTimerBuilder::default().build();
    let task_instance_chain = delay_timer.insert_task(build_task_async_print()?)?;
    let task_instance = task_instance_chain.next_with_wait()?;
    task_instance.cancel_with_wait()?;
    delay_timer.remove_task(1)?;
    delay_timer.stop_delay_timer()?;

    Ok(())
}

fn build_task_async_print() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    let body = || async {
        println!("create_async_fn_body!");
        time::sleep(Duration::from_secs(3)).await;
        println!("create_async_fn_body:i'success");
    };

    task_builder
        .set_task_id(1)
        .set_frequency_repeated_by_cron_str("@secondly")
        .set_maximum_parallel_runnable_num(2)
        .spawn_async_routine(body)
}
