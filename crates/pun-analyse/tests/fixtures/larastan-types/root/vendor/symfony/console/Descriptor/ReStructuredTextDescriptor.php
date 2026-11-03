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
use Symfony\Component\String\UnicodeString;

class ReStructuredTextDescriptor extends Descriptor
{
    // <h1>
    private string $partChar = '=';
    // <h2>
    private string $chapterChar = '-';
    // <h3>
    private string $sectionChar = '~';
    // <h4>
    private string $subsectionChar = '.';
    // <h5>
    private string $subsubsectionChar = '^';
    // <h6>
    private string $paragraphsChar = '"';

    private array $visibleNamespaces = [];

    public function describe(OutputInterface $output, object $object, array $options = []): void
    {}

    /**
     * Override parent method to set $decorated = true.
     */
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

    private function describeCommands($application, array $options): void
    {}

    private function createTableOfContents(ApplicationDescription $description, Application $application): void
    {}

    private function getNonDefaultOptions(InputDefinition $definition): array
    {}

    private function setVisibleNamespaces(ApplicationDescription $description): void
    {}

    private function removeAliasesAndHiddenCommands(array $commands): array
    {}
}
