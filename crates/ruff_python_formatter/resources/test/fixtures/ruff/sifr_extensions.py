# Sifr formatter extension coverage.


def consume(mut own items: list[int], own fallback: list[int]) -> list[int]:
    # `mut own` must format through the Ruff AST rule as canonical `own mut`.
    match len(items):
        case 0:
            return fallback
        case _:
            return [value for value in items if value > 0]


class Box[T]:
    value: T

    def replace(own mut self, own value: T) -> T:
        old: T = self.value
        self.value = value
        return old


def shape(values: dict[str, list[int]]) -> Result[list[int], str]:
    if "items" not in values:
        return Err("missing")
    return Ok(values["items"])


def documented(mut own values: list[int]) -> list[int]:
    """
    Sifr docstring snippets format when docstring code formatting is enabled.

    ```sifr
    def sample( mut own items:list[int])->list[int]:
        return [item for item in items if item>0]
    ```

    .. code-block:: sifr

        def passthrough( own item:Result[int,str])->Result[int,str]:
            return item
    """
    return values


def suppressed(mut borrowed: list[int]) -> list[int]:
    # fmt: off
    untouched=[ 3,2,1]
    # fmt: on
    return borrowed
