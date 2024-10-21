import * as z from 'zod';
import memoizeOne from 'memoize-one';

export const CurrentUser = memoizeOne(() => z.object({email: z.string(), }));
export type CurrentUser = z.infer<ReturnType<typeof CurrentUser>>;

export const Message = memoizeOne(() => z.object({message: z.string(), }));
export type Message = z.infer<ReturnType<typeof Message>>;

