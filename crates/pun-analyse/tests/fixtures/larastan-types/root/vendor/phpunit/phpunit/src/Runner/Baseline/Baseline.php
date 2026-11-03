<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Baseline;

use function ksort;
use function strcmp;
use function usort;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Baseline
{
    public const int VERSION = 1;

    /**
     * @var array<non-empty-string, array<positive-int, list<Issue>>>
     */
    private array $issues = [];

    public function add(Issue $issue): void
    {}

    public function has(Issue $issue): bool
    {}

    /**
     * @return array<non-empty-string, array<positive-int, list<Issue>>>
     */
    public function groupedByFileAndLine(): array
    {}
}
