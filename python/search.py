from pathlib import Path

import click
from interactive_search_page import InteractiveSearchPagerApp, render_content_blocks
from rich.console import Console
from scan_search import Client, Query
from scan_search.config import generate_default_config
from utils.resolve_config import resolve_config

console = Console()


@click.command()
@click.argument("file_names", type=click.Path(exists=True), nargs=-1)
@click.option("-r", "--reload", is_flag=True, help="Reload extracted text")
@click.option("-s", "--search", type=str, help="Phrase to search for in the file")
@click.option("-sm", "--semsearch", type=str, help="Phrase for semantic search")
@click.option("-i", "--interactive", is_flag=True, help="Interactive search mode")
@click.option("-a", "--after", "context_after", type=int, help="Context after for search")
@click.option("-b", "--before", "context_before", type=int, help="Context before for search")
@click.option("-c", "--context", type=int, help="Context around for search")
@click.option("-fl", "--follow-links", is_flag=True, default=None, help="Follow links for search")
@click.option("-ih", "--include-hidden", is_flag=True, default=None, help="Include hidden links")
@click.option("-m", "--model", type=str, help="Encoder ML model for semsearch")
@click.option("-l", "--languages", type=tuple[str], help="Languages to used for ocr")
@click.option("--default-config", is_flag=True, default=None, help="Generate default config")
@click.option(
    "--config",
    "config_path",
    type=click.Path(path_type=Path, exists=True),
    help="Config to use",
)
def cli(
    file_names: tuple[str, ...],
    reload: bool,
    search: str | None,
    semsearch: str | None,
    interactive: bool,
    context: int | None,
    context_before: int | None,
    context_after: int | None,
    follow_links: bool | None,
    include_hidden: bool | None,
    model: str | None,
    languages: tuple[str] | None,
    default_config: bool,
    config_path: Path | None,
):
    if default_config:
        generate_default_config()

    config = resolve_config(
        semsearch is not None,
        context_after,
        context_before,
        context,
        follow_links,
        include_hidden,
        model,
        languages,
        config_path,
    )

    console.print("[bold green]Loading files...[/bold green]")

    with console.status("[bold green]Creating cache...[/bold green]", spinner="dots"):
        client = Client(config, list(file_names))

    if reload:
        click.echo("Text extraction reloaded")
        if not any([search, semsearch, interactive]):
            return
    if search and not interactive:
        click.echo(f"Search phrase: {search}")
        with console.status("[bold green]Searching...[/bold green]", spinner="dots"):
            search_data = client.search(
                Query(search, before=config.search.context_before, after=config.search.context_after)
            )
        view_results(search_data)

    if semsearch and not interactive:
        click.echo(f"Semantic search phrase: {semsearch}")
        with console.status("[bold green]Searching...[/bold green]", spinner="dots"):
            search_data = client.sem_search(
                Query(semsearch, before=config.search.context_before, after=config.search.context_after)
            )
        view_results(search_data)

    if interactive:
        click.echo("Interactive search mode")
        search_mode = "semantic" if semsearch else "normal"
        initial_query = semsearch if semsearch else search

        view_interactive(
            client,
            config.search.context_before,
            config.search.context_after,
            mode=search_mode,
            initial_query=initial_query,
        )


def view_results(results: list):
    content = render_content_blocks(results)
    with console.pager(styles=True):
        console.print(content)


def view_interactive(client, context_before: int, context_after: int, mode="normal", initial_query=None):

    pager = InteractiveSearchPagerApp(
        "",
        client=client,
        context_before=context_before,
        context_after=context_after,
        mode=mode,
        initial_query=initial_query,
    )
    pager.run()


if __name__ == "__main__":
    cli()
