<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output\Compact;

use const PHP_EOL;
use function assert;
use function str_replace;
use function trim;
use PHPUnit\Event\Code\Test;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\Util\Sanitizer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Renderer
{
    private Printer $printer;

    public function __construct(Printer $printer)
    {}

    public function nameOfTest(Test $test): string
    {}

    /**
     * Prints the header line that starts a record ("--- TYPE: title").
     *
     * The header must always be exactly one line so that a user-supplied title cannot
     * start a record of its own. Line feeds and carriage returns in the title are therefore
     * made visible as escape sequences, in addition to the control characters that
     * Sanitizer::sanitizeControlCharacters() makes visible.
     */
    public function printHeader(string $type, string $title): void
    {}

    /**
     * Prints one or more lines of user-supplied text that belong to the current record.
     */
    public function printBody(string $body): void
    {}

    public function printThrowable(Throwable $throwable): void
    {}

    public function printStackTrace(string $stackTrace): void
    {}

    private function singleLine(string $text): string
    {}
}
