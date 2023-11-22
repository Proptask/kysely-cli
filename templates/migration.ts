import { Kysely, sql } from 'kysely';
import { DB } from '../models';

export async function up(db: Kysely<DB>): Promise<void> {
  // await db.schema
  //   .createTable('<tableName>')
  //   .addColumn('id', 'uuid', (col) =>
  //     col.primaryKey().defaultTo(sql`gen_random_uuid()`),
  //   )
  //   .addColumn('name', 'varchar', (col) => col.notNull())
  //   .addColumn('createdAt', 'timestamp', (col) =>
  //     col.defaultTo(sql`now()`).notNull(),
  //   )
  //   // .addColumn('<referenceId>', 'uuid', (col) =>
  //   //   col.references('<tableName>.id').onDelete('cascade').notNull(),
  //   // )
  //   .addColumn('deletedAt', 'timestamp')
  //   .execute();

  // await db.schema
  //   .createIndex('<indexName>')
  //   .on('<tableName>')
  //   .column('<column>')
  //   .execute();
}

export async function down(db: Kysely<DB>): Promise<void> {
  // await db.schema.dropTable('<tableName>').cascade().execute();
}
