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

use function assert;
use PHPUnit\Framework\MockObject\Rule\InvocationOrder;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This trait is not covered by the backward compatibility promise for PHPUnit
 */
trait MockObjectApi
{
    public function __phpunit_hasInvocationCountRule(): bool
    {}

    public function __phpunit_hasParametersRule(): bool
    {}

    public function __phpunit_verify(bool $unsetInvocationMocker = true): void
    {}

    abstract public function __phpunit_state(): TestDoubleState;

    abstract public function __phpunit_getInvocationHandler(): InvocationHandler;

    abstract public function __phpunit_unsetInvocationMocker(): void;

    public function __phpunit_recordsInvocations(): bool
    {}

    public function expects(InvocationOrder $matcher): InvocationMocker
    {}

    /**
     * @param ?non-empty-string                         $label
     * @param array<non-empty-string, non-empty-string> $methodLabels
     *
     * @throws EmptyInvocationJournalLabelException
     * @throws InvocationJournalAlreadyRegisteredException
     * @throws MethodCannotBeConfiguredException
     * @throws TestDoubleSealedException
     */
    public function recordInvocationsIn(InvocationJournal $journal, ?string $label = null, array $methodLabels = []): void
    {}
}
