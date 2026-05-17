import click
from scan_search import process_files


def create_cache(file_names: tuple[str, ...]):
    word_map = process_files(*file_names)
    click.echo(word_map)


@click.command()
@click.argument("file_names", type=click.Path(exists=True), nargs=-1)
@click.option("-r", "--reload", is_flag=True, help="Reload extracted text")
@click.option("-s", "--search", type=str, help="Phrase to search for in the file")
@click.option("-sm", "--semsearch", type=str, help="Phrase for semantic search")
@click.option("-si", "--interactive", is_flag=True, help="Interactive search mode")
def cli(file_names: tuple[str, ...], reload: bool, search: str | None, semsearch: str | None, interactive: bool):
    if reload:
        click.echo("Text extraction reloaded")

    if not any([search, semsearch, interactive]):
        create_cache(file_names)
        return

    if search:
        click.echo(f"Search phrase: {search}")
    if semsearch:
        click.echo(f"Semantic search phrase: {semsearch}")
    if interactive:
        click.echo("Interactive search mode")


if __name__ == "__main__":
    cli()
