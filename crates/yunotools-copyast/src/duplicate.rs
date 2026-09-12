use crate::{DuplicateGroup, TextFile};
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct DuplicateDetector;

impl DuplicateDetector {
    // Tìm những nhóm file có nội dung giống hệt nhau
    pub fn find(files: &[TextFile]) -> Vec<DuplicateGroup> {
        exact_duplicate_groups(files)
            .into_iter()
            .map(|indices| create_duplicate_group(files, &indices))
            .collect()
    }

    // Xóa các file trùng khỏi Vec
    // File đầu tiên trong mỗi nhóm được giữ lại.
    // Function trả về số file đã xóa
    pub fn remove_duplicates(files: &mut Vec<TextFile>) -> usize {
        // tương đương
        // let groups = exact_duplicate_groups(files);
        //
        // let mut duplicate_indices = HashSet::new();
        //
        // for group in groups {
        //     for index in group.into_iter().skip(1) {
        //         duplicate_indices.insert(index);
        //     }
        // }
        let duplicate_indices = exact_duplicate_groups(files)
            .into_iter()
            // Giả sử exact_duplicate_groups() trả về:
            // vec![
            //     vec![0, 2, 5],
            //     vec![1, 4],
            // ]
            // Điều này có nghĩa:
            // File 0, 2, 5 giống nhau
            // File 1, 4 giống nhau
            // Trong mỗi nhóm, ta giữ phần tử đầu tiên:
            // [0, 2, 5] → giữ 0, xóa 2 và 5
            // [1, 4]    → giữ 1, xóa 4
            .flat_map(|group| {
                // Bỏ index đầu tiên vì file đầu tiên
                // trong nhóm sẽ được giữ lại
                group.into_iter().skip(1)
            })
            .collect::<HashSet<_>>();

        let removed_count = duplicate_indices.len();

        let mut current_index = 0_usize;

        // retain() duyệt từng phần tử trong vector:
        // - Closure trả true → giữ phần tử.
        // - Closure trả false → xóa phần tử.
        files.retain(|_| {
            let should_keep = !duplicate_indices.contains(&current_index);

            current_index += 1;

            should_keep
        });

        removed_count
    }
}

// Tìm index của các file trùng nhau
// Ví dụ kết quả:
// [
//     [0, 3, 5],
//     [2, 7],
// ]
//
// Nghĩa là:
// - file 0, 3 và 5 giống nhau;
// - file 2 và 7 giống nhau

// Các file
//    ↓
// Nhóm theo (kích thước, hash)
//    ↓
// So sánh nội dung thật
//    ↓
// Các nhóm duplicate chính xác
fn exact_duplicate_groups(files: &[TextFile]) -> Vec<Vec<usize>> {
    let mut candidates = HashMap::<(u64, u64), Vec<usize>>::new();

    // 1: gom những file có cùng kích thước và hash vào 1 nhóm ứng viên
    for (index, file) in files.iter().enumerate() {
        let key = (file.size_bytes(), calculate_content_hash(&file.content));

        candidates.entry(key).or_default().push(index);
    }

    let mut duplicate_groups = Vec::new();

    // 2: so sánh nội dung thật để loại trừ trường hợp 2 file vô tình có cùng hash
    for candidate_group in candidates.into_values() {
        if candidate_group.len() < 2 {
            continue;
        }

        let exact_groups = split_hash_collisions(files, candidate_group);

        for group in exact_groups {
            if group.len() > 1 {
                duplicate_groups.push(group);
            }
        }
    }

    // HashMap không bảo đảm thứ tự.
    // Sort giúp kết quả ổn định giữa các lần chạy
    duplicate_groups.sort_by(|left, right| files[left[0]].path.cmp(&files[right[0]].path));

    duplicate_groups
}

// Tạo fingerprint nhanh cho nội dung file
// Hash này chỉ dùng để tìm ứng viên duplicate,
// không dùng cho mật khẩu hoặc bảo mật.
fn calculate_content_hash(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

// Tách những file có cùng hash nhưng nội dung khác nhau
// Hash collision hiếm nhưng vẫn có thể xảy ra
fn split_hash_collisions(files: &[TextFile], candidate_group: Vec<usize>) -> Vec<Vec<usize>> {
    let mut exact_groups = Vec::<Vec<usize>>::new();

    // 'candidate là tên của vòng lặp
    // Đây không phải lifetime dù cú pháp khá giống lifetime
    'candidate: for index in candidate_group {
        for exact_group in &mut exact_groups {
            // Lấy file đầu tiên làm đại diện cho cả group
            let first_index = exact_group[0];

            if files[first_index].content == files[index].content {
                exact_group.push(index);
                continue 'candidate;
            }
        }

        // Chưa có nhóm nào có cùng nội dung,
        // tạo một nhóm mới
        exact_groups.push(vec![index]);
    }

    exact_groups
}

// Chuyển nhóm index thành DuplicateGroup
fn create_duplicate_group(files: &[TextFile], indices: &[usize]) -> DuplicateGroup {
    let paths = indices
        .iter()
        .map(|&index| files[index].path.clone())
        .collect::<Vec<_>>();

    let duplicate_count = indices.len().saturating_sub(1);

    let duplicate_count = u64::try_from(duplicate_count).unwrap_or(u64::MAX);

    let redundant_bytes = files[indices[0]]
        .size_bytes()
        .saturating_mul(duplicate_count);

    DuplicateGroup {
        paths,
        redundant_bytes,
    }
}
