use futures::StreamExt;
use jj_lib::merged_tree::{MergedTree, TreeDiffStream};
use pollster::FutureExt;
                let tree_diff = from_tree.diff_stream(to_tree, matcher);
                let tree_diff = from_tree.diff_stream(to_tree, matcher);
                let tree_diff = from_tree.diff_stream(to_tree, matcher);
                let tree_diff = from_tree.diff_stream(to_tree, matcher);
                let tree_diff = from_tree.diff_stream(to_tree, matcher);
            conflicts::materialize(value, repo.store(), path, &mut content)
                .block_on()
                .unwrap();
    mut tree_diff: TreeDiffStream,
    async {
        while let Some((path, diff)) = tree_diff.next().await {
            let ui_path = workspace_command.format_file_path(&path);
            let (left_value, right_value) = diff?;
            if left_value.is_absent() {
                let right_content = diff_content(repo, &path, &right_value)?;
                let description = basic_diff_file_type(&right_value);
                writeln!(
                    formatter.labeled("header"),
                    "Added {description} {ui_path}:"
                )?;
                if right_content.is_empty() {
                    writeln!(formatter.labeled("empty"), "    (empty)")?;
                } else {
                    show_color_words_diff_hunks(&[], &right_content, formatter)?;
            } else if right_value.is_present() {
                let left_content = diff_content(repo, &path, &left_value)?;
                let right_content = diff_content(repo, &path, &right_value)?;
                let description = match (left_value.into_resolved(), right_value.into_resolved()) {
                    (
                        Ok(Some(TreeValue::File {
                            executable: left_executable,
                            ..
                        })),
                        Ok(Some(TreeValue::File {
                            executable: right_executable,
                            ..
                        })),
                    ) => {
                        if left_executable && right_executable {
                            "Modified executable file".to_string()
                        } else if left_executable {
                            "Executable file became non-executable at".to_string()
                        } else if right_executable {
                            "Non-executable file became executable at".to_string()
                        } else {
                            "Modified regular file".to_string()
                        }
                    }
                    (Err(_), Err(_)) => "Modified conflict in".to_string(),
                    (Err(_), _) => "Resolved conflict in".to_string(),
                    (_, Err(_)) => "Created conflict in".to_string(),
                    (Ok(Some(TreeValue::Symlink(_))), Ok(Some(TreeValue::Symlink(_)))) => {
                        "Symlink target changed at".to_string()
                    }
                    (Ok(left_value), Ok(right_value)) => {
                        let left_type = basic_diff_file_type(&Merge::resolved(left_value));
                        let right_type = basic_diff_file_type(&Merge::resolved(right_value));
                        let (first, rest) = left_type.split_at(1);
                        format!(
                            "{}{} became {} at",
                            first.to_ascii_uppercase(),
                            rest,
                            right_type
                        )
                    }
                };
                writeln!(formatter.labeled("header"), "{description} {ui_path}:")?;
                show_color_words_diff_hunks(&left_content, &right_content, formatter)?;
                let left_content = diff_content(repo, &path, &left_value)?;
                let description = basic_diff_file_type(&left_value);
                writeln!(
                    formatter.labeled("header"),
                    "Removed {description} {ui_path}:"
                )?;
                if left_content.is_empty() {
                    writeln!(formatter.labeled("empty"), "    (empty)")?;
                } else {
                    show_color_words_diff_hunks(&left_content, &[], formatter)?;
                }
        Ok::<(), CommandError>(())
    .block_on()?;
            conflicts::materialize(value, repo.store(), path, &mut content)
                .block_on()
                .unwrap();
    mut tree_diff: TreeDiffStream,
    async {
        while let Some((path, diff)) = tree_diff.next().await {
            let path_string = path.to_internal_file_string();
            let (left_value, right_value) = diff?;
            if left_value.is_absent() {
                let right_part = git_diff_part(repo, &path, &right_value)?;
                formatter.with_label("file_header", |formatter| {
                    writeln!(formatter, "diff --git a/{path_string} b/{path_string}")?;
                    writeln!(formatter, "new file mode {}", &right_part.mode)?;
                    writeln!(formatter, "index 0000000000..{}", &right_part.hash)?;
                    writeln!(formatter, "--- /dev/null")?;
                    writeln!(formatter, "+++ b/{path_string}")
                })?;
                show_unified_diff_hunks(formatter, &[], &right_part.content)?;
            } else if right_value.is_present() {
                let left_part = git_diff_part(repo, &path, &left_value)?;
                let right_part = git_diff_part(repo, &path, &right_value)?;
                formatter.with_label("file_header", |formatter| {
                    writeln!(formatter, "diff --git a/{path_string} b/{path_string}")?;
                    if left_part.mode != right_part.mode {
                        writeln!(formatter, "old mode {}", &left_part.mode)?;
                        writeln!(formatter, "new mode {}", &right_part.mode)?;
                        if left_part.hash != right_part.hash {
                            writeln!(formatter, "index {}...{}", &left_part.hash, right_part.hash)?;
                        }
                    } else if left_part.hash != right_part.hash {
                        writeln!(
                            formatter,
                            "index {}...{} {}",
                            &left_part.hash, right_part.hash, left_part.mode
                        )?;
                    if left_part.content != right_part.content {
                        writeln!(formatter, "--- a/{path_string}")?;
                        writeln!(formatter, "+++ b/{path_string}")?;
                    }
                    Ok(())
                })?;
                show_unified_diff_hunks(formatter, &left_part.content, &right_part.content)?;
            } else {
                let left_part = git_diff_part(repo, &path, &left_value)?;
                formatter.with_label("file_header", |formatter| {
                    writeln!(formatter, "diff --git a/{path_string} b/{path_string}")?;
                    writeln!(formatter, "deleted file mode {}", &left_part.mode)?;
                    writeln!(formatter, "index {}..0000000000", &left_part.hash)?;
                    writeln!(formatter, "+++ /dev/null")
                })?;
                show_unified_diff_hunks(formatter, &left_part.content, &[])?;
            }
        Ok::<(), CommandError>(())
    .block_on()?;
    mut tree_diff: TreeDiffStream,
    formatter.with_label("diff", |formatter| -> io::Result<()> {
        async {
            while let Some((repo_path, diff)) = tree_diff.next().await {
                let (before, after) = diff.unwrap();
                if before.is_present() && after.is_present() {
                    writeln!(
                        formatter.labeled("modified"),
                        "M {}",
                        workspace_command.format_file_path(&repo_path)
                    )?;
                } else if before.is_absent() {
                    writeln!(
                        formatter.labeled("added"),
                        "A {}",
                        workspace_command.format_file_path(&repo_path)
                    )?;
                } else {
                    writeln!(
                        formatter.labeled("removed"),
                        "R {}",
                        workspace_command.format_file_path(&repo_path)
                    )?;
                }
            Ok(())
        .block_on()
    mut tree_diff: TreeDiffStream,

    async {
        while let Some((repo_path, diff)) = tree_diff.next().await {
            let (left, right) = diff?;
            let path = workspace_command.format_file_path(&repo_path);
            let left_content = diff_content(workspace_command.repo(), &repo_path, &left)?;
            let right_content = diff_content(workspace_command.repo(), &repo_path, &right)?;
            max_path_width = max(max_path_width, path.width());
            let stat = get_diff_stat(path, &left_content, &right_content);
            max_diffs = max(max_diffs, stat.added + stat.removed);
            stats.push(stat);
        }
        Ok::<(), CommandError>(())
    .block_on()?;
    mut tree_diff: TreeDiffStream,
        async {
            while let Some((repo_path, diff)) = tree_diff.next().await {
                let (before, after) = diff.unwrap();
                writeln!(
                    formatter.labeled("modified"),
                    "{}{} {}",
                    diff_summary_char(&before),
                    diff_summary_char(&after),
                    workspace_command.format_file_path(&repo_path)
                )?;
            }
            Ok(())
        .block_on()