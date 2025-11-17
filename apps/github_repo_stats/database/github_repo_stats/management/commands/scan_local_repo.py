import pathlib
import re
import subprocess

from django.core.management.base import BaseCommand, CommandError
from github_repo_stats.models import Commit, Contributor, Repository, User

COMMIT_MARK = "---COMMIT-MARK---"


class Command(BaseCommand):
    help = "Populates the database with some data (only use for testing!)"

    def add_arguments(self, parser):
        parser.add_argument("server", type=str)
        parser.add_argument("org", type=str)
        parser.add_argument("name", type=str)
        parser.add_argument("local_path", type=pathlib.Path)

    def handle(self, *args, **options):
        self.stdout.write("Running 'scan_local_repo' command")
        self.stdout.write(f" - server: {options['server']}")
        self.stdout.write(f" - org: {options['org']}")
        self.stdout.write(f" - name: {options['name']}")
        self.stdout.write(f" - local_path: {options['local_path']}")

        repo, _ = Repository.objects.get_or_create(
            server=options["server"],
            org=options["org"],
            name=options["name"],
        )

        log_output = self._get_git_logs(path=options["local_path"])
        commits = list(map(str.strip, log_output.split(COMMIT_MARK)))
        total = len(commits)
        self.stdout.write(f"Found {len(commits)} commits in the 'main' branch")

        stat_re = re.compile(
            r"^\s*(\d+) files? changed(?:, (\d+) insertions?\(\+\))?(?:, (\d+) deletions?\(\-\))?$"
        )

        for i, commit in enumerate(commits):
            self.stdout.write(f"[{i}/{total}]")
            commit = commit.strip()
            if not commit:
                continue

            lines = commit.split("\n")
            if len(lines) < 4:
                self.stdout.write(f" - skipped commit {i}:\n{commit}\n")

                # self.stdout.write(f" - next commit {i}:\n{commits[i+1]}")
                # continue
                return

            commit_sha_line = lines[0].strip()
            self.stdout.write(f" - commit sha: {commit_sha_line}")

            author_line = lines[1].strip()
            author_match = re.match(r"^(.*?)\s*<(\S+?)>$", author_line)
            if not author_match:
                continue
            author_name = author_match.group(1).strip()
            author_email = author_match.group(2).strip()
            self.stdout.write(f" - author_name: {author_name}")
            self.stdout.write(f" - author_email: {author_email}")

            title_line = lines[2].strip()
            self.stdout.write(f" - title_line: {title_line}")
            pr_number_match = re.match(r"^.*?\(#(\d+)\)$", title_line)
            if pr_number_match:
                pr_number = pr_number_match.group(1).strip()
                self.stdout.write(f" - pr_number: {pr_number}")

            short_stats_line = lines[-1].strip()
            self.stdout.write(f" - short_stats_line: {short_stats_line}")
            stat_match = stat_re.match(short_stats_line)
            if stat_match:
                ins = int(stat_match.group(2) or 0)
                dels = int(stat_match.group(3) or 0)
                changes = ins + dels
            else:
                changes = 0
            self.stdout.write(f" - changes: {changes}")

            # Find co-authors
            coauthors = re.findall(r"Co-authored-by:\s*.*\s*<(\S+?)>", commit, re.IGNORECASE)
            self.stdout.write(f" - coauthors: {coauthors}")

            # All contributors
            contributors = [author_email] + [co.strip() for co in coauthors if co.strip()]
            unique_contribs = set(contributors)
            self.stdout.write(f" - unique_contribs: {unique_contribs}")

            # Populate the commit
            commit, commit_created = Commit.objects.get_or_create(
                repo=repo,
                sha=commit_sha_line,
                defaults={
                    "changes": changes,
                    "pull_request": pr_number,
                },
            )
            # assert commit_created, f"Remove the repo and recreate it again"

            #  Work on the contributors // users
            all_users = []
            for it in unique_contribs:
                email = it.lower()
                c, c_created = Contributor.objects.get_or_create(email=email)
                if c_created or not c.user:
                    user = User.objects.create(name=email)
                    c.user = user
                    c.save()
                else:
                    user = c.user

                all_users.append(user)

            used = set()
            unique_users = [x for x in all_users if x.id not in used and (used.add(x) or True)]

            for user in unique_users:
                commit.contributors.add(user)
            commit.save()

    def _get_git_logs(self, path: pathlib.Path):
        # Get git logs for 2025 in a parseable format with stats
        cmd = [
            "git",
            "log",
            "main",
            "--since=2025-01-01",
            "--until=2026-01-01",  # Covers all of 2025
            "--shortstat",
            f"--format={COMMIT_MARK}%n%H%n%an <%ae>%n%s%n%B",
        ]
        try:
            output = subprocess.check_output(cmd, cwd=path).decode("utf-8", errors="ignore")
            return output
        except subprocess.CalledProcessError as e:
            raise CommandError(f"Error running git log: {e}")
