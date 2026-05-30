import click
from interactive_search_page import InteractiveSearchPagerApp, render_content_blocks
from rich.console import Console
from scan_search import Client, Query, process_files


def create_cache(file_names: tuple[str, ...]):
    word_map = process_files(*file_names)
    click.echo(word_map)


console = Console()


@click.command()
@click.argument("file_names", type=click.Path(exists=True), nargs=-1)
@click.option("-r", "--reload", is_flag=True, help="Reload extracted text")
@click.option("-s", "--search", type=str, help="Phrase to search for in the file")
@click.option("-sm", "--semsearch", type=str, help="Phrase for semantic search")
@click.option("-i", "--interactive", is_flag=True, help="Interactive search mode")
@click.option("-c", "--context", type=int, help="Context for search")
@click.option("-fl", "--follow-links", is_flag=True, help="Follow links for search")
@click.option("-ih", "--include-hidden", is_flag=True, help="Include hidden links")
@click.option("-m", "--model", type=str, help="Encoder ML model for semsearch")
@click.option("-l", "--languages", type=tuple[str], help="Models to used for ocr")
def cli(
    file_names: tuple[str, ...],
    reload: bool,
    search: str | None,
    semsearch: str | None,
    interactive: bool,
    context: int | None,
    follow_links: bool,
    include_hidden: bool,
    model: str | None,
    languages: tuple[str] | None,
):

    console.print("[bold green]Loading files...[/bold green]")
    with console.status("[bold green]Creating cache...[/bold green]", spinner="dots"):
        client = Client(list(file_names))
    if reload:
        click.echo("Text extraction reloaded")
        if not any([search, semsearch, interactive]):
            create_cache(file_names)
            return
    if context is None:
        context = 5

    if search and not interactive:
        click.echo(f"Search phrase: {search}")
        with console.status("[bold green]Searching...[/bold green]", spinner="dots"):
            search_data = client.search(Query(search, before=context, after=context))
        view_results(search_data)

    if semsearch and not interactive:
        click.echo(f"Semantic search phrase: {semsearch}")
        with console.status("[bold green]Searching...[/bold green]", spinner="dots"):
            search_data = client.sem_search(Query(semsearch, before=context, after=context))
        view_results(search_data)

    if interactive:
        click.echo("Interactive search mode")
        search_mode = "semantic" if semsearch else "normal"
        initial_query = semsearch if semsearch else search

        view_interactive(client, context, mode=search_mode, initial_query=initial_query)


def view_results(results: list):
    content = render_content_blocks(results)
    with console.pager(styles=True):
        console.print(content)


def view_interactive(client, context: int, mode="normal", initial_query=None):

    pager = InteractiveSearchPagerApp("", client=client, context=context, mode=mode, initial_query=initial_query)
    pager.run()


if __name__ == "__main__":
    cli()
