import test from 'ava'

import { open, close } from '../index.js'

test('works', (t) => {
	t.is(!!open, true);
	t.is(!!close, true);
})
