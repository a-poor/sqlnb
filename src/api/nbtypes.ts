/**
 * Notebook cell data.
 */
export interface INotebook {
    filepath: string;
    dbpath: string;
    cells: (IMarkdownCell | IQueryCell)[];
}

/**
 * Content of a markdown cell.
 */
export interface IMarkdownCell {
    type: "markdown";
    content: string;
}

/**
 * Content of a SQL query cell.
 */
export interface IQueryCell {
    type: "query";
    query: string;
    results?: {
        [colName: string]: (string | boolean | number | null)[]
    };
    queryTime?: number;
    queryError?: string;
}

