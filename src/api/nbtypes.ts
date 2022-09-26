/**
 * Notebook cell data.
 */
export interface INotebook {
    filepath: string;
    filename: string;
    connection: IConnection;
    cells: (IMarkdownCell | IQueryCell)[];
}

/**
 * SQL connection data.
 */
export interface IConnection {
}

/**
 * Content of a markdown cell.
 */
export interface IMarkdownCell {
    type: "text";
    content: string;
}

/**
 * Content of a SQL query cell.
 */
export interface IQueryCell {
    type: "query";
    query: string;
    runAt?: Date;
    runTime?: number;
    results?: {
        [colName: string]: (string | boolean | number | null)[]
    };
}

