<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Filter;

use function assert;
use function preg_match;
use function restore_error_handler;
use function set_error_handler;
use function sprintf;
use function substr;

/**
 * The value of the --filter CLI option, compiled to the regular expressions
 * used to select tests.
 *
 * This is the only place where the syntax of that option is parsed. The regular
 * expression that is matched against the name of a test, including the name of
 * its data set, and the regular expression that is matched against the name of
 * a test method alone are derived from the same parse so that they cannot
 * disagree.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class CompiledNameFilter
{
    /**
     * @var non-empty-string
     */
    private string $regularExpression;

    /**
     * @var ?non-empty-string
     */
    private ?string $methodNameRegularExpression;
    private ?int $dataSetMinimum;
    private ?int $dataSetMaximum;

    public static function from(string $filter): self
    {}

    /**
     * @param non-empty-string  $regularExpression
     * @param ?non-empty-string $methodNameRegularExpression
     */
    private function __construct(string $regularExpression, ?string $methodNameRegularExpression, ?int $dataSetMinimum, ?int $dataSetMaximum)
    {}

    /**
     * The regular expression to match against the name of a test, including the
     * name of its data set.
     *
     * @return non-empty-string
     */
    public function regularExpression(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->methodNameRegularExpression
     */
    public function constrainsMethodName(): bool
    {}

    /**
     * The regular expression to match against the name of a test method, not
     * including the name of a data set.
     *
     * This is only available when the filter has a data set portion. A filter
     * without one is matched against the entire name of a test, including the
     * name of its data set, and may therefore match a data set name that cannot
     * be known without invoking the data provider (#6741).
     *
     * @return non-empty-string
     */
    public function methodNameRegularExpression(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->dataSetMinimum
     * @phpstan-assert-if-true !null $this->dataSetMaximum
     */
    public function hasDataSetRange(): bool
    {}

    public function dataSetMinimum(): int
    {}

    public function dataSetMaximum(): int
    {}

    /**
     * A filter that does not begin with an alphanumeric character and that is a
     * valid regular expression is used as-is instead of being parsed.
     */
    private static function isFilterSyntax(string $filter): bool
    {}

    private static function isRegularExpression(string $filter): bool
    {}
}
