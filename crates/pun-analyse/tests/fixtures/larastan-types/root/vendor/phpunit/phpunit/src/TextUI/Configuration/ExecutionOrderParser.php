<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use function explode;
use function in_array;
use function sprintf;
use PHPUnit\Event\Emitter;
use PHPUnit\Runner\TestSuiteSorter;

/**
 * Parses the comma-separated list of tokens that the --order-by CLI option and
 * the executionOrder XML configuration attribute accept.
 *
 * This is the single place where those tokens are given meaning, so that both
 * configuration surfaces agree on how a value is interpreted and on which
 * values are diagnosed.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ExecutionOrderParser
{
    /**
     * Tokens that select the order in which tests are sorted. At most one of
     * them is meaningful.
     *
     * @var non-empty-list<non-empty-string>
     */
    private const array MAIN_ORDER_TOKENS = [
        'default',
        'duration',
        'duration-ascending',
        'duration-descending',
        'modified-ascending',
        'modified-descending',
        'random',
        'reverse',
        'size',
        'size-ascending',
        'size-descending',
    ];
    private Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @param string $value the configured value; every token it does not give
     *                      meaning to, including the empty token, is reported
     *                      back to the caller
     */
    public function parse(string $value, ExecutionOrderSource $source, ?int $executionOrder, ?int $executionOrderDefects, ?bool $resolveDependencies): ExecutionOrder
    {}

    /**
     * @param non-empty-string $token
     * @param non-empty-string $replacement
     */
    private function deprecateRenamedToken(ExecutionOrderSource $source, string $token, string $replacement): void
    {}

    /**
     * Whether dependencies between tests are resolved is not an ordering
     * strategy, and it has had a dedicated configuration option all along.
     *
     * @param non-empty-string $token
     * @param non-empty-string $replacement
     */
    private function deprecateDependencyToken(ExecutionOrderSource $source, string $token, string $replacement): void
    {}

    /**
     * @param non-empty-string $previousToken
     * @param non-empty-string $token
     */
    private function deprecateMultipleOrders(ExecutionOrderSource $source, string $previousToken, string $token): void
    {}

    /**
     * @param non-empty-string $token
     */
    private function deprecateDefectsBeforeOrder(ExecutionOrderSource $source, string $token): void
    {}

    /**
     * The replacement suggested by a diagnostic must never be a token that is
     * itself deprecated.
     *
     * @param non-empty-string $token
     *
     * @return non-empty-string
     */
    private function canonicalOrderToken(string $token): string
    {}
}
