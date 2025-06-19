use crate::grid::{Cell, HEIGHT, WIDTH};
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),   // 节点在网格中的坐标
    cost: usize,                // 从起点到该节点的实际代价(g(n))
    heuristic: usize,           // 该节点到终点的预估代价(h(n))
}

// 实现 Ord 和 PartialOrd trait, 让 Node 能够被放入优先队列

// 反向比较，BinaryHeap优先处理大的数，所以通过Ord trait将小的数包装成大的，让BinaryHeap优先处理
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_path(
    grid: &[[Cell; WIDTH]; HEIGHT],
    start: (usize, usize),
    target: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new(); // 候选路线，类型：BinaryHeap-优先级排列
    let mut closed_set = HashSet::new(); // 已探索区域, 快速判断是否已访问（O(1)查询）
    let mut came_from = std::collections::HashMap::new(); // 足迹地图 - 记录每个位置的来源, 快速存储/查询位置关系（O(1)）
    let mut g_score = std::collections::HashMap::new(); // 记录到各点的实际代价

    g_score.insert(start, 0);
    // 优先级队列。BinaryHeap类型。每次 push 或 pop 时，堆都会自动调用 Ord trait 的方法来维护堆性质
    open_set.push(Node {
        position: start,
        cost: 0,
        heuristic: manhattan_distance(start, target),
    });

    // 循环从优先级队列取出 f值最小 的节点
    while let Some(current) = open_set.pop() {

        // 找到目标后，回溯构建完整路径（根据之前记录的「足迹地图」倒推出最佳路线）
        if current.position == target {
            // 重建路径
            let mut path = Vec::new();
            let mut current_pos = target;

            while current_pos != start {
                path.push(current_pos);
                current_pos = came_from[&current_pos]; // came_from 存储每个位置的前驱节点
            }

            path.reverse(); // 反转路径
            return Some(path);
        }

        closed_set.insert(current.position); // 标记已探索

        // 处理邻居节点
        for neighbor in get_neighbors(grid, current.position) {
            if closed_set.contains(&neighbor) {
                continue; // 跳过已探索的
            }

            let tentative_g_score = g_score[&current.position] + 1;

            if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                came_from.insert(neighbor, current.position);  // 记录路径来源
                g_score.insert(neighbor, tentative_g_score);   // 更新实际代价

                open_set.push(Node {  // 加入待探索队列
                    position: neighbor,
                    cost: tentative_g_score,
                    heuristic: manhattan_distance(neighbor, target),
                });
            }
        }
    }

    None
}

fn get_neighbors(grid: &[[Cell; WIDTH]; HEIGHT], pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (x, y) = pos;

    // 上
    if x > 0 && grid[x - 1][y] != Cell::Obstacle {
        neighbors.push((x - 1, y));
    }

    // 下
    if x < HEIGHT - 1 && grid[x + 1][y] != Cell::Obstacle {
        neighbors.push((x + 1, y));
    }

    // 左
    if y > 0 && grid[x][y - 1] != Cell::Obstacle {
        neighbors.push((x, y - 1));
    }

    // 右
    if y < WIDTH - 1 && grid[x][y + 1] != Cell::Obstacle {
        neighbors.push((x, y + 1));
    }

    neighbors
}

// 启发式函数: 计算的是两个点之间的曼哈顿距离  = |x1 - x2| + |y1 - y2|
fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}
