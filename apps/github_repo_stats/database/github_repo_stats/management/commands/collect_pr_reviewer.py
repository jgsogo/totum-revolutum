import os

import requests
from django.core.management.base import BaseCommand, CommandError
from github_repo_stats.models import Commit


class Command(BaseCommand):
    help = "Populates the database with some data (only use for testing!)"

    def add_arguments(self, parser):
        parser.add_argument("server", type=str)
        parser.add_argument("org", type=str)
        parser.add_argument("name", type=str)

    def handle(self, *args, **options):
        self.stdout.write("Running 'scan_repo' command")
        server = options["server"]
        org = options["org"]
        name = options["name"]
        self.stdout.write(f" - server: {server}")
        self.stdout.write(f" - org: {org}")
        self.stdout.write(f" - name: {name}")

        GITHUB_TOKEN = os.getenv("GITHUB_TOKEN")
        if not GITHUB_TOKEN:
            raise CommandError("GITHUB_TOKEN envvar was not provided")

        headers = {"Authorization": f"Bearer {GITHUB_TOKEN}"}
        URL_PATTERN = f"{server}/repos/{org}/{name}/pulls/{{pr_number}}/reviews"

        all_commits = Commit.objects.filter(pull_request__isnull=False)
        for commit in all_commits:
            url = URL_PATTERN.format(pr_number=commit.pull_request)
            response = requests.get(url, headers=headers)
            response.raise_for_status()
            self.stdout.write(response.text)  # FIXME: do not print
            reviews = response.json()

            reviewers = set()
            for review in reviews:
                if review.get("user"):
                    reviewers.add(review["user"]["login"])

            self.stdout.write(list(reviewers))
