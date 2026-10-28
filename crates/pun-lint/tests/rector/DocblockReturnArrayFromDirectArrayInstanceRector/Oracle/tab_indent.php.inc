<?php

class TabIndent
{
	/**
	 * Tabbed
	 */
	public function run(): array
	{
		return [1.5];
	}
}
-----
<?php

class TabIndent
{
	/**
     * Tabbed
     * @return array<int, float>
     */
    public function run(): array
	{
		return [1.5];
	}
}
