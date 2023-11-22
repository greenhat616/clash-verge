//! A "prelude" for users of the `delay-timer` crate.
//!
//! This prelude is similar to the standard library's prelude in that you'll
//! almost always want to import its entire contents, but unlike the standard
//! library's prelude you'll have to do so manually:
//!
//! ```
//! use delay_timer::prelude::*;
//! ```
//!
//! The prelude may grow over time as additional items see ubiquitous use.

pub use crate::{
    entity::{timestamp, timestamp_micros, DelayTimer, DelayTimerBuilder},
    error::*,
    timer::{
        runtime_trace::{
            state::instance,
            task_handle::DelayTaskHandler,
            task_instance::{Instance, TaskInstance, TaskInstancesChain},
        },
        task::{
            FrequencyCronStr as Frequency, ScheduleIteratorTimeZone, Task, TaskBuilder, TaskContext,
        },
        timer_core::{FinishOutput, FinishTaskBody, TimerEvent},
    },
};

pub use crate::utils::convenience::{
    cron_expression_grammatical_candy::{CandyCron, CandyCronStr, CandyFrequency},
    functions::{create_default_delay_task_handler, create_delay_task_handler},
};

pub use anyhow::{anyhow, Result as AnyResult};
pub use cron_clock::{self, error as cron_error, FixedOffset, Local, TimeZone, Utc};
pub use smol::{
    channel, future as future_lite, spawn as async_spawn_by_smol, unblock as unblock_spawn_by_smol,
    Task as SmolJoinHandler,
};
pub use thiserror::Error;

/// State of the task run instance.
pub type InstanceState = usize;

pub(crate) use crate::{
    entity::RuntimeKind,
    timer::{
        event_handle::SharedHeader,
        runtime_trace::{
            state,
            task_handle::{DelayTaskHandlerBox, DelayTaskHandlerBoxBuilder},
            task_instance::TaskInstancesChainMaintainer,
        },
        task::Routine,
    },
};

pub(crate) use crate::utils::parse::shell_command::{ChildGuard, ChildGuardList, ChildUnify};
pub(crate) use dashmap::DashMap;
pub(crate) use log::{debug, error, trace};
pub(crate) use smol::{
    channel::{unbounded, Receiver as AsyncReceiver, Sender as AsyncSender},
    future::yield_now,
    lock::Mutex as AsyncMutex,
    Timer as AsyncTimer,
};
pub(crate) use std::{
    convert::{TryFrom, TryInto},
    future::Future,
    iter::StepBy,
    ops::RangeFrom,
    process::ExitStatus,
    time::Duration,
};
pub(crate) use tracing::{info_span, instrument, Instrument};

pub(crate) type SecondsState = StepBy<RangeFrom<u64>>;
pub(crate) type TimerEventSender = AsyncSender<TimerEvent>;
pub(crate) type TimerEventReceiver = AsyncReceiver<TimerEvent>;

pub use tokio::{
    task::{
        spawn as async_spawn_by_tokio, spawn_blocking as unblock_spawn_by_tokio,
        JoinHandle as TokioJoinHandle,
    },
    time::sleep as sleep_by_tokio,
};

cfg_status_report!(
    pub use crate::utils::status_report::PublicEvent;
    pub(crate) use crate::utils::status_report::GLOBAL_STATUS_REPORTER;
);

#[cfg(target_family = "unix")]
pub(crate) use std::os::unix::process::ExitStatusExt;
#[cfg(target_family = "windows")]
pub(crate) use std::os::windows::process::ExitStatusExt;

pub(crate) const ONE_SECOND: u64 = 1;
pub(crate) const ONE_MINUTE: u64 = ONE_SECOND * 60;
pub(crate) const ONE_HOUR: u64 = ONE_MINUTE * 60;
pub(crate) const ONE_DAY: u64 = ONE_HOUR * 24;
