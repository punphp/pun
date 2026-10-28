<?php

final class StringKeysExistingDoc
{
    /**
     * Process the data.
     *
     * @return void
     */
    public function process(array $data)
    {
        $name = $data['name'];
    }
}
-----
<?php

final class StringKeysExistingDoc
{
    /**
     * Process the data.
     *
     * @return void
     * @param array<string, mixed> $data
     */
    public function process(array $data)
    {
        $name = $data['name'];
    }
}
