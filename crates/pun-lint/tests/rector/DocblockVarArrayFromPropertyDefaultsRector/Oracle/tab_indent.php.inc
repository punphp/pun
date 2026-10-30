<?php

final class TabIndent
{
	private array $flags = [true, false];
}
-----
<?php

final class TabIndent
{
	/**
     * @var bool[]
     */
    private array $flags = [true, false];
}
