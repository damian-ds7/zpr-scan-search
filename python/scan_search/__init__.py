import atexit

from .scan_search import *  # noqa: F403

__doc__ = scan_search.__doc__  # noqa: F405
if hasattr(scan_search, "__all__"):  # noqa: F405
    __all__ = scan_search.__all__  # noqa: F405

atexit.register(flush_cache_writer)
