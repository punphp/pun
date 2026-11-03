<?php declare(strict_types=1);
/*
 * This file is part of sebastian/global-state.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\GlobalState;

use const PHP_EOL;
use function is_array;
use function is_scalar;
use function serialize;
use function sprintf;
use function var_export;

final class CodeExporter
{
    public function constants(Snapshot $snapshot): string
    {}

    public function globalVariables(Snapshot $snapshot): string
    {}

    public function iniSettings(Snapshot $snapshot): string
    {}

    private function exportVariable(mixed $variable): string
    {}

    /**
     * @param array<mixed> $array
     */
    private function arrayOnlyContainsScalars(array $array): bool
    {}
}
