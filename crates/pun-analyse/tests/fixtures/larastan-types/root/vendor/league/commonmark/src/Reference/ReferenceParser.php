<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * Original code based on the CommonMark JS reference parser (https://bitly.com/commonmark-js)
 *  - (c) John MacFarlane
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Reference;

use League\CommonMark\Parser\Cursor;
use League\CommonMark\Util\LinkParserHelper;

final class ReferenceParser
{
    // Looking for the start of a definition, i.e. `[`
    private const START_DEFINITION = 0;
    // Looking for and parsing the label, i.e. `[foo]` within `[foo]`
    private const LABEL = 1;
    // Parsing the destination, i.e. `/url` in `[foo]: /url`
    private const DESTINATION = 2;
    // Looking for the start of a title, i.e. the first `"` in `[foo]: /url "title"`
    private const START_TITLE = 3;
    // Parsing the content of the title, i.e. `title` in `[foo]: /url "title"`
    private const TITLE = 4;
    // End state, no matter what kind of lines we add, they won't be references
    private const PARAGRAPH = 5;

    /** @psalm-readonly-allow-private-mutation */
    private string $paragraph = '';

    /**
     * @var array<int, ReferenceInterface>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $references = [];

    /** @psalm-readonly-allow-private-mutation */
    private int $state = self::START_DEFINITION;

    /** @psalm-readonly-allow-private-mutation */
    private ?string $label = null;

    /** @psalm-readonly-allow-private-mutation */
    private ?string $destination = null;

    /**
     * @var string string
     *
     * @psalm-readonly-allow-private-mutation
     */
    private string $title = '';

    /** @psalm-readonly-allow-private-mutation */
    private ?string $titleDelimiter = null;

    /** @psalm-readonly-allow-private-mutation */
    private bool $referenceValid = false;

    public function getParagraphContent(): string
    {}

    /**
     * @return ReferenceInterface[]
     */
    public function getReferences(): iterable
    {}

    public function hasReferences(): bool
    {}

    public function parse(string $line): void
    {}

    private function parseStartDefinition(Cursor $cursor): bool
    {}

    private function parseLabel(Cursor $cursor): bool
    {}

    private function parseDestination(Cursor $cursor): bool
    {}

    private function parseStartTitle(Cursor $cursor): bool
    {}

    private function parseTitle(Cursor $cursor): bool
    {}

    private function finishReference(): void
    {}
}
