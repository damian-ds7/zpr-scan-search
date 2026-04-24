import click

@click.command()
@click.argument("file_name", type=click.Path())
@click.option("-s", "--search", type=str, help="Phrase to search for in the file.")
@click.option("-sm", "--semsearch", type=str, help="Phrase for semantic search.")
@click.option("-r", "--reload", is_flag=True, help="Reload extracted text")
def cli(file_name: str, search: str | None, semsearch: str | None, reload: bool):
    click.echo(f"File name: {file_name}")
    if reload:
        click.echo("Text extraction reloaded")
    if search:
        click.echo(f"Search phrase: {search}")
    if semsearch:
        click.echo(f"Semantic search phrase: {semsearch}")

if __name__ == "__main__":
    cli()
