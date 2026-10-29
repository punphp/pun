<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class Service {}

final class ContainerTest extends TestCase
{
    /** @var Service */
    private $selfContainer;
    /** @var Service */
    private $otherGetContainer;
    /** @var Service */
    private $otherContainerProperty;
    /** @var Service */
    private $upperCase;
    /** @var Service */
    private $otherName;
    /** @var Service */
    private $nullsafe;
    /** @var Service */
    private $nullsafeCaller;
    /** @var Service */
    private $otherVariable;
    /** @var Service */
    private $dynamicMethod;
    /** @var Service */
    private $dynamicCaller;
    /** @var Service */
    private $notThis;
    /** @var Service */
    private $inClosure;
    /** @var Service */
    private $compound;
    /** @var Service */
    private $staticOther;
    /** @var Service */
    private $notCall;
    /** @var \App\Service */
    private $fqcn;

    protected function setUp(): void
    {
        $this->selfContainer = self::getContainer()->get('a');
        $this->otherGetContainer = $this->kernel->getContainer()->get('a');
        $this->otherContainerProperty = $this->kernel->container->get('a');
        $this->UPPERCASE = $this->container->GET('a');
        $this->otherName = $this->container->fetch('a');
        $this->nullsafe = $this->container?->get('a');
        $this->nullsafeCaller = $this->kernel?->getContainer()->get('a');
        $this->otherVariable = $container->get('a');
        $this->dynamicMethod = $this->container->$name('a');
        $this->dynamicCaller = $this->{$c}->get('a');
        $other->notThis = $this->get('a');
        $fn = function () {
            $this->inClosure = $this->get('a');
        };
        $this->compound .= $this->get('a');
        $this->staticOther = static::boot()->get('a');
        $this->notCall = $this->container;
        $this->fqcn = $this->get('a');
    }
}

final class WithoutSetUp extends TestCase
{
    /** @var Service */
    private $service;

    protected function prepare(): void
    {
        $this->service = $this->get('a');
    }
}
-----
<?php

namespace App;

use PHPUnit\Framework\TestCase;

final class Service {}

final class ContainerTest extends TestCase
{
    private \App\Service $selfContainer;
    private \App\Service $otherGetContainer;
    private \App\Service $otherContainerProperty;
    private \App\Service $upperCase;
    /** @var Service */
    private $otherName;
    /** @var Service */
    private $nullsafe;
    /** @var Service */
    private $nullsafeCaller;
    /** @var Service */
    private $otherVariable;
    /** @var Service */
    private $dynamicMethod;
    /** @var Service */
    private $dynamicCaller;
    /** @var Service */
    private $notThis;
    private \App\Service $inClosure;
    /** @var Service */
    private $compound;
    /** @var Service */
    private $staticOther;
    /** @var Service */
    private $notCall;
    private \App\Service $fqcn;

    protected function setUp(): void
    {
        $this->selfContainer = self::getContainer()->get('a');
        $this->otherGetContainer = $this->kernel->getContainer()->get('a');
        $this->otherContainerProperty = $this->kernel->container->get('a');
        $this->UPPERCASE = $this->container->GET('a');
        $this->otherName = $this->container->fetch('a');
        $this->nullsafe = $this->container?->get('a');
        $this->nullsafeCaller = $this->kernel?->getContainer()->get('a');
        $this->otherVariable = $container->get('a');
        $this->dynamicMethod = $this->container->$name('a');
        $this->dynamicCaller = $this->{$c}->get('a');
        $other->notThis = $this->get('a');
        $fn = function () {
            $this->inClosure = $this->get('a');
        };
        $this->compound .= $this->get('a');
        $this->staticOther = static::boot()->get('a');
        $this->notCall = $this->container;
        $this->fqcn = $this->get('a');
    }
}

final class WithoutSetUp extends TestCase
{
    /** @var Service */
    private $service;

    protected function prepare(): void
    {
        $this->service = $this->get('a');
    }
}
