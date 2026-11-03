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
use Symfony\Component\Console\Helper\Helper;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\OutputInterface;

/**
 * Markdown descriptor.
 *
 * @author Jean-François Simon <contact@jfsimon.fr>
 *
 * @internal
 */
class MarkdownDescriptor extends Descriptor
{
    public function describe(OutputInterface $output, object $object, array $options = []): void
    {}

    protected function write(string $content, bool $decorated = true): void
    {}

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

    private function getApplicationTitle(Application $application): string
    {}
}
