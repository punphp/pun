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

use const PATH_SEPARATOR;
use function constant;
use function define;
use function defined;
use function getenv;
use function implode;
use function ini_get;
use function ini_set;
use function is_array;
use function is_scalar;
use function putenv;
use function restore_error_handler;
use function set_error_handler;
use function sprintf;
use PHPUnit\Event\Emitter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class PhpHandler
{
    private Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    public function handle(Php $configuration): void
    {}

    private function handleIncludePaths(DirectoryCollection $includePaths): void
    {}

    private function handleIniSettings(IniSettingCollection $iniSettings): void
    {}

    private function handleConstants(ConstantCollection $constants): void
    {}

    private function handleGlobalVariables(VariableCollection $variables): void
    {}

    private function handleServerVariables(VariableCollection $variables): void
    {}

    private function handleVariables(string $target, VariableCollection $variables): void
    {}

    private function handleEnvVariables(VariableCollection $variables): void
    {}
}
