<?php

namespace Laravel\Prompts;

use Closure;
use Illuminate\Support\Collection;

class DataTablePrompt extends Prompt
{
    use Concerns\Scrolling;
    use Concerns\TypedValue;

    /**
     * The table headers.
     *
     * @var array<int, string|array<int, string>>
     */
    public array $headers;

    /**
     * The table rows.
     *
     * @var array<int|string, array<int, string>>
     */
    public array $rows;

    /**
     * The cached filtered rows.
     *
     * @var array<int|string, array<int, string>>|null
     */
    protected ?array $filteredCache = null;

    /**
     * The previous search query (for cache invalidation).
     */
    protected string $previousQuery = '';

    /**
     * Create a new DataTable instance.
     *
     * @param  array<int, string|array<int, string>>|Collection<int, string|array<int, string>>  $headers
     * @param  array<int|string, array<int, string>>|Collection<int|string, array<int, string>>|null  $rows
     *
     * @phpstan-param ($rows is null ? list<list<string>>|Collection<int, list<string>> : list<string|list<string>>|Collection<int, string|list<string>>) $headers
     */
    public function __construct(
        array|Collection $headers = [],
        array|Collection|null $rows = null,
        public int $scroll = 10,
        public string $label = '',
        public string $hint = '',
        public bool|string $required = false,
        public mixed $validate = null,
        public ?Closure $transform = null,
        public ?Closure $filter = null,
    ) {}

    /**
     * Handle key presses in browse mode.
     */
    protected function handleBrowseKey(string $key): void
    {}

    /**
     * Handle key presses in search mode.
     */
    protected function handleSearchKey(string $key): void
    {}

    /**
     * Enter search mode.
     */
    protected function enterSearch(): void
    {}

    /**
     * Exit search mode, keeping the filtered results.
     */
    protected function exitSearch(): void
    {}

    /**
     * Cancel search, clearing the query and showing all rows.
     */
    protected function cancelSearch(): void
    {}

    /**
     * Handle typing in search mode.
     */
    protected function search(): void
    {}

    /**
     * Get the filtered rows based on the current search query.
     *
     * @return array<int|string, array<int, string>>
     */
    public function filteredRows(): array
    {}

    /**
     * The currently visible rows.
     *
     * @return array<int|string, array<int, string>>
     */
    public function visible(): array
    {}

    /**
     * Get the current search query.
     */
    public function searchValue(): string
    {}

    /**
     * Get the search query with a virtual cursor.
     */
    public function searchWithCursor(int $maxWidth): string
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): mixed
    {}

    /**
     * Get the selected row for display purposes.
     *
     * @return array<int, string>|null
     */
    public function selectedRow(): ?array
    {}
}
