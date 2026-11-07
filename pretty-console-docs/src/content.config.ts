import { defineCollection } from 'astro:content';
import { z } from 'astro/zod';
import { docsLoader, i18nLoader } from '@astrojs/starlight/loaders';
import { docsSchema, i18nSchema } from '@astrojs/starlight/schema';

const docs = defineCollection({
	loader: docsLoader(),
	schema: docsSchema({
		extend: z.object({
			/** The Diátaxis quadrant this page belongs to. The splash homepage carries no type. */
			type: z.enum(['tutorial', 'how-to', 'reference', 'explanation']).optional(),
		}),
	}),
});

const i18n = defineCollection({
	loader: i18nLoader(),
	schema: i18nSchema(),
});

export const collections = { docs, i18n };
