use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub display_path: String,
    pub score: u32,
    pub is_dir: bool,
}

pub struct FileSearcher {
    matcher: Matcher,
}

impl FileSearcher {
    pub fn new() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT),
        }
    }

    pub fn search(&mut self, base_dir: &Path, query: &str, max_results: usize, dir_only: bool, exact: bool) -> Vec<SearchResult> {
        if query.is_empty() {
            return Vec::new();
        }

        let is_path_query = query.contains('/');
        let query_lower = query.to_lowercase();

        // クエリの最後のセグメントを取得（パスクエリ用）
        let query_last_segment = if is_path_query {
            query.rsplit('/').next().unwrap_or(query)
        } else {
            query
        };
        let query_last_segment_lower = query_last_segment.to_lowercase();

        // ファジーマッチ用パターン（exactモードでは使わない）
        let pattern = if !exact {
            Some(Pattern::new(
                query,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Fuzzy,
            ))
        } else {
            None
        };

        let mut results: Vec<SearchResult> = Vec::new();

        let walker = WalkBuilder::new(base_dir)
            .hidden(false)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .max_depth(Some(10))
            .build();

        for entry in walker.flatten() {
            let path = entry.path();
            let is_dir = path.is_dir();

            // ディレクトリのみモードの場合、ファイルをスキップ
            if dir_only && !is_dir {
                continue;
            }

            // ファイル/ディレクトリ名を取得
            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };

            // ベースディレクトリからの相対パスを取得（表示用）
            let display_path = path
                .strip_prefix(base_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            if display_path.is_empty() {
                continue;
            }

            let file_name_lower = file_name.to_lowercase();

            if exact {
                // 完全一致モード：ファイル名がクエリと完全一致（大文字小文字無視）
                let matches = if is_path_query {
                    // パスクエリの場合：パスにクエリが含まれ、かつファイル名が最後のセグメントと完全一致
                    let display_path_lower = display_path.to_lowercase();
                    display_path_lower.contains(&query_lower) && file_name_lower == query_last_segment_lower
                } else {
                    // 通常：ファイル名がクエリと完全一致
                    file_name_lower == query_lower
                };

                if matches {
                    results.push(SearchResult {
                        path: path.to_path_buf(),
                        display_path,
                        score: 1000, // 完全一致は固定スコア
                        is_dir,
                    });
                }
            } else {
                // ファジーマッチモード
                let target = if is_path_query { &display_path } else { &file_name };
                let mut buf = Vec::new();
                let haystack = Utf32Str::new(target, &mut buf);

                if let Some(ref pat) = pattern {
                    if let Some(score) = pat.score(haystack, &mut self.matcher) {
                        // パスクエリの場合、ファイル名がクエリの最後のセグメントを含まないものは除外
                        if is_path_query && !file_name_lower.contains(&query_last_segment_lower) {
                            continue;
                        }

                        results.push(SearchResult {
                            path: path.to_path_buf(),
                            display_path,
                            score,
                            is_dir,
                        });
                    }
                }
            }
        }

        // スコアで降順ソート
        results.sort_by(|a, b| b.score.cmp(&a.score));
        results.truncate(max_results);
        results
    }
}
