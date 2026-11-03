<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI;

use const PHP_EOL;
use function count;
use function defined;
use function explode;
use function max;
use function preg_replace_callback;
use function str_pad;
use function str_repeat;
use function strlen;
use function wordwrap;
use PHPUnit\Util\Color;
use SebastianBergmann\Environment\Console;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Help
{
    private const string LEFT_MARGIN       = '  ';
    private int $lengthOfLongestOptionName = 0;
    private readonly int $columnsAvailableForDescription;
    private bool $hasColor;

    public function __construct(?int $width = null, ?bool $withColor = null)
    {}

    public function generate(): string
    {}

    private function writeWithoutColor(): string
    {}

    private function writeWithColor(): string
    {}

    /**
     * @return array<non-empty-string, non-empty-list<array{arg: non-empty-string, desc: non-empty-string}|array{spacer: ''}|array{text: non-empty-string}>>
     */
    private function elements(): array
    {}
}
