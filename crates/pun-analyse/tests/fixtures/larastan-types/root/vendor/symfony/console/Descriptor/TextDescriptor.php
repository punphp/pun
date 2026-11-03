<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Descriptor;

use Symfony\Component\Console\Application;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Formatter\OutputFormatter;
use Symfony\Component\Console\Helper\Helper;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputOption;

/**
 * Text descriptor.
 *
 * @author Jean-François Simon <contact@jfsimon.fr>
 *
 * @internal
 */
class TextDescriptor extends Descriptor
{
    protected function describeInputArgument(InputArgument $argument, array $options = []): void
    {}

    protected function describeInputOption(InputOption $option, array $options = []): void
    {}

    protected function describeInputDefinition(InputDefinition $definition, array $options = []): void
    {}

    protected function describeCommand(Command $command, array $options = []): void
    {}

    protected function describeApplication(Application $application, array $options = []): void
    {}

    private function writeText(string $content, array $options = []): void
    {}

    /**
     * Formats command aliases to show them in the command description.
     */
    private function getCommandAliasesText(Command $command): string
    {}

    /**
     * Formats input option/argument default value.
     */
    private function formatDefaultValue(mixed $default): string
    {}

    /**
     * @param array<Command|string> $commands
     */
    private function getColumnWidth(array $commands): int
    {}

    /**
     * @param InputOption[] $options
     */
    private function calculateTotalWidthForOptions(array $options): int
    {}
}
