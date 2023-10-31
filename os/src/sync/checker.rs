const MAX_RESOURCE_COUNT: usize = 256;
const MAX_THREAD_COUNT: usize = 256;

struct DeadlockChecker {
    /// 资源数量
    available: [usize; MAX_RESOURCE_COUNT],
    /// 线程已分配资源数量
    allocation: [[usize; MAX_RESOURCE_COUNT]; MAX_THREAD_COUNT],
    /// 线程还需要分配资源数量
    need: [[usize; MAX_RESOURCE_COUNT]; MAX_THREAD_COUNT],
}

impl DeadlockChecker {
    pub fn new() -> Self {
        Self {
            available: [0; MAX_RESOURCE_COUNT],
            allocation: [[0; MAX_RESOURCE_COUNT]; MAX_THREAD_COUNT],
            need: [[0; MAX_RESOURCE_COUNT]; MAX_THREAD_COUNT],
        }
    }


    /// 设置资源可用数量
    pub fn set_resource(&mut self, id: usize, num: usize) {
        self.available[id] = num;
    }

    /// 释放thread占用资源
    pub fn release_resource(&mut self, thread_id: usize, resource_id: usize) {
        self.available[resource_id] += 1;
        self.allocation[thread_id][resource_id] -= 1;
    }

    /// 申请资源
    pub fn check_resource(&mut self, tid: usize, rid: usize) -> bool {
        self.need[tid][rid] += 1;
        let mut work = self.available;
        let mut finish = [false; MAX_THREAD_COUNT];
        for i in 0..MAX_THREAD_COUNT {
            let mut count = 0;
            for j in 0..MAX_RESOURCE_COUNT {
                if self.need[i][j] <= work[j] {
                    count += 1;
                    if count = MAX_RESOURCE_COUNT {
                        for j in 0..MAX_RESOURCE_COUNT {
                            work[j] += self.alloc[i][j];
                        }
                        finish[i] = true;
                    }
                }
            }
        }
        self.need[tid][rid] -= 1;
        return if (0..MAX_RESOURCE_COUNT).into_iter().all(|idx| finish[idx]) {
            self.allocation[tid][rid] += 1;
            self.available[rid] -= 1;
            true
        } else {
            false
        };
    }
}