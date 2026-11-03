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
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputOption;

/**
 * JSON descriptor.
 *
 * @author Jean-François Simon <contact@jfsimon.fr>
 *
 * @internal
 */
class JsonDescriptor extends Descriptor
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

    /**
     * Writes data as json.
     */
    private function writeData(array $data, array $options): void
    {}

    private function getInputArgumentData(InputArgument $argument): array
    {}

    private function getInputOptionData(InputOption $option, bool $negated = false): array
    {}

    private function getInputDefinitionData(InputDefinition $definition): array
    {}

    private function getCommandData(Command $command, bool $short = false): array
    {}
}
