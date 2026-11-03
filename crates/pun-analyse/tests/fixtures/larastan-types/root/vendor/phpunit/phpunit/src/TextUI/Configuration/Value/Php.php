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

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 */
final readonly class Php
{
    private DirectoryCollection $includePaths;
    private IniSettingCollection $iniSettings;
    private ConstantCollection $constants;
    private VariableCollection $globalVariables;
    private VariableCollection $envVariables;
    private VariableCollection $postVariables;
    private VariableCollection $getVariables;
    private VariableCollection $cookieVariables;
    private VariableCollection $serverVariables;
    private VariableCollection $filesVariables;
    private VariableCollection $requestVariables;

    public function __construct(DirectoryCollection $includePaths, IniSettingCollection $iniSettings, ConstantCollection $constants, VariableCollection $globalVariables, VariableCollection $envVariables, VariableCollection $postVariables, VariableCollection $getVariables, VariableCollection $cookieVariables, VariableCollection $serverVariables, VariableCollection $filesVariables, VariableCollection $requestVariables)
    {}

    public function includePaths(): DirectoryCollection
    {}

    public function iniSettings(): IniSettingCollection
    {}

    public function constants(): ConstantCollection
    {}

    public function globalVariables(): VariableCollection
    {}

    public function envVariables(): VariableCollection
    {}

    public function postVariables(): VariableCollection
    {}

    public function getVariables(): VariableCollection
    {}

    public function cookieVariables(): VariableCollection
    {}

    public function serverVariables(): VariableCollection
    {}

    public function filesVariables(): VariableCollection
    {}

    public function requestVariables(): VariableCollection
    {}
}
