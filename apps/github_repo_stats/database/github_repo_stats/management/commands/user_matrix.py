from collections import defaultdict
from itertools import combinations

from django.core.management.base import BaseCommand
from github_repo_stats.models import Commit, Repository


class Command(BaseCommand):
    help = "Populates the database with some data (only use for testing!)"

    def add_arguments(self, parser):
        parser.add_argument("server", type=str)
        parser.add_argument("org", type=str)
        parser.add_argument("name", type=str)
        parser.add_argument("out_data", choices=["interactions", "changes"], type=str)

    def handle(self, *args, **options):
        self.stdout.write("Running 'user_matrix' command")
        self.stdout.write(f" - server: {options['server']}")
        self.stdout.write(f" - org: {options['org']}")
        self.stdout.write(f" - name: {options['name']}")

        # Get all the commits for the repo
        repo = Repository.objects.get(
            server=options["server"],
            org=options["org"],
            name=options["name"],
        )
        all_commits = Commit.objects.filter(repo=repo)

        user_matrix_interactions = defaultdict(int)
        user_matrix_changes = defaultdict(int)
        for commit in all_commits:
            contributors = commit.contributors.all().order_by("id")
            if len(contributors) > 1:
                for u1, u2 in combinations(contributors, 2):
                    user_matrix_interactions[(u1.name, u2.name)] += 1
                    user_matrix_changes[(u1.name, u2.name)] += commit.changes
            else:
                assert len(contributors) == 1
                c = contributors[0]
                user_matrix_interactions[(c, c)] += 1
                user_matrix_changes[(c, c)] += commit.changes

        # Print interactions
        if options["out_data"] == "interactions":
            self.stdout.write("INTERACTIONS")
            all_data = [((u1, u2), v) for (u1, u2), v in user_matrix_interactions.items()]
            all_data = sorted(all_data, key=lambda u: u[1])
            for (u1, u2), v in all_data:
                self.stdout.write(f"{u1}\t{u2}\t{v}")
                if u1 != u2:
                    self.stdout.write(f"{u2}\t{u1}\t{v}")

        # Print changes
        if options["out_data"] == "changes":
            self.stdout.write("CHANGES")
            all_data = [((u1, u2), v) for (u1, u2), v in user_matrix_changes.items()]
            all_data = sorted(all_data, key=lambda u: u[1])
            for (u1, u2), v in all_data:
                self.stdout.write(f"{u1}\t{u2}\t{v}")
                if u1 != u2:
                    self.stdout.write(f"{u2}\t{u1}\t{v}")
