<?php

namespace Laravel\Prompts\Themes\Default;

use InvalidArgumentException;
use Laravel\Prompts\Callout;
use Laravel\Prompts\Elements\BulletedList;
use Laravel\Prompts\Elements\ElementContract;
use Laravel\Prompts\Elements\Heading;
use Laravel\Prompts\Elements\KeyValueList;
use Laravel\Prompts\Elements\Link;
use Laravel\Prompts\Elements\NumberedList;

class CalloutRenderer extends Renderer
{
    use Concerns\DrawsBoxes;

    /**
     * Render the text prompt.
     */
    public function __invoke(Callout $prompt): string
    {}

    /**
     * Resolve a part of the callout content into a string or array of strings.
     */
    /**
     * @return string|array<int, string>
     */
    protected function resolvePart(string|ElementContract $part): string|array
    {}

    /**
     * Render a bulleted list element.
     *
     * @return array<int, string>
     */
    protected function renderBulletedList(BulletedList $part): array
    {}

    /**
     * Render a numbered list element.
     *
     * @return array<int, string>
     */
    protected function renderNumberedList(NumberedList $part): array
    {}

    /**
     * Render a key-value list element.
     *
     * @return array<int, string>
     */
    protected function renderKeyValueList(KeyValueList $part): array
    {}

    /**
     * Render a link element.
     */
    protected function renderLink(Link $part): string
    {}

    /**
     * Auto-format the text by applying styles to specific patterns, such as inline code blocks.
     */
    protected function autoFormat(string $text): string
    {}
}
