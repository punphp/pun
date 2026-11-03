<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject;

use function count;
use function in_array;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class InvocationJournalImplementation implements InvocationJournalInternal
{
    /**
     * @var list<non-empty-string>
     */
    private array $invocations = [];

    /**
     * @param non-empty-string $label
     */
    public function record(string $label): void
    {}

    /**
     * @return list<non-empty-string>
     */
    public function asArray(): array
    {}

    /**
     * @param non-empty-string $label
     * @param non-empty-string ...$additionalLabels
     *
     * @return list<non-empty-string>
     */
    public function only(string $label, string ...$additionalLabels): array
    {}

    public function count(): int
    {}

    public function getIterator(): InvocationJournalIterator
    {}
}
