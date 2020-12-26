from typing import Any, Dict, List


def filter_dict(locals: Dict[str, Any], keys: List[str]):
    return dict((key, locals[key])
                for key in keys)
