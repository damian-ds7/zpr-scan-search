from pathlib import Path

from rich.text import Text
from scan_search import Client, Query, SearchResult
from textual.app import App, ComposeResult
from textual.containers import VerticalScroll
from textual.widgets import Input, Static


def render_content_blocks(results: list[tuple[Path, list[SearchResult]]]):
    content_blocks = []

    for result in results:
        search_results = result[1]
        if len(search_results) == 0:
            continue
        block = Text()
        block.append(f"\n---{result[0].name}---\n", style="bold violet")

        for search_result in search_results:
            block.append("...", style="bold red")
            block.append(f"{search_result.before()} ")
            block.append(search_result.matched(), style="bold blue")
            block.append(f" {search_result.after()}")
            block.append("...", style="bold red")

        content_blocks.append(block)

    if not content_blocks:
        return "No results found."

    output = Text()
    for i, block in enumerate(content_blocks):
        if i > 0:
            output.append("\n\n")
        output.append_text(block)

    return output


class InteractiveSearchPagerApp(App):
    """
    This custom pager is used for interactive search, both normal and semantic
    It provides a search bar and a pager like scroll
    """

    CSS = """
    Screen {
        background: $background;
    }
    #scroll-view {
        height: 1fr;
    }
    #viewer {
        padding: 0 1;
        width: 100%;
    }
    #help-line {
        color: $text-muted;
        background: transparent;
        height: 1;
        padding: 0 1;
    }
    #search-bar {
        border: none;
        background: transparent;
        height: 1;
        padding: 0 1;
        margin: 0;
    }
    #search-bar:focus {
        border: none;
    }
    """

    BINDINGS = [
        ("q", "quit", "Quit"),
        ("/", "search_mode", "Search"),
        ("escape", "normal_mode", "Cancel Search"),
    ]

    def __init__(
        self,
        initial_text: str,
        client: Client,
        context_before: int,
        context_after: int,
        mode="normal",
        initial_query: str | None = None,
    ):
        super().__init__()
        self.initial_text = (
            initial_text if initial_text else "[dim]Pager workspace empty. Press [/] to start a new search...[/dim]"
        )
        self.client = client
        self.context_before = context_before
        self.context_after = context_after
        self.mode = mode
        self.initial_query = initial_query

    def compose(self) -> ComposeResult:
        with VerticalScroll(id="scroll-view"):
            yield Static(self.initial_text, id="viewer")
        yield Static("Press [/] to search, [q] to quit", id="help-line", markup=False)
        yield Input(placeholder="", id="search-bar")

    def on_mount(self) -> None:
        self.query_one("#search-bar").visible = False
        self.query_one("#help-line").visible = True
        self.query_one("#scroll-view").focus()

        if self.initial_query:
            self.search(self.initial_query)

    def action_search_mode(self) -> None:
        search_bar = self.query_one("#search-bar", Input)
        help = self.query_one("#help-line", Static)

        help.visible = False
        search_bar.visible = True

        search_bar.placeholder = "Search..."

        search_bar.value = "/"
        search_bar.focus()

    def action_normal_mode(self) -> None:
        search_bar = self.query_one("#search-bar", Input)
        help_line = self.query_one("#help-line", Static)

        search_bar.visible = False
        help_line.visible = True
        search_bar.value = ""
        self.query_one("#scroll-view").focus()

    def on_input_submitted(self, event: Input.Submitted) -> None:
        search_phrase = event.value.lstrip("/")
        self.search(search_phrase)
        self.action_normal_mode()

    def search(self, search_phrase: str) -> None:
        viewer = self.query_one("#viewer", Static)

        if not search_phrase:
            viewer.update(self.initial_text)
            return

        if self.mode == "semantic":
            results = self.client.sem_search(Query(search_phrase, before=self.context_before, after=self.context_after))
        else:
            results = self.client.search(Query(search_phrase, before=self.context_before, after=self.context_after))

        content = render_content_blocks(results)
        viewer.update(content)
