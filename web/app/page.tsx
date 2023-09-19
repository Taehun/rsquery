'use client';

import { MagnifyingGlassCircleIcon, PlayIcon } from "@heroicons/react/24/solid";
import {
  Button, Card, Tab, TabGroup, TabList, TabPanel, TabPanels,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeaderCell,
  TableRow,
  Text,
  Title
} from "@tremor/react";
import CodeEditor from '@uiw/react-textarea-code-editor';
import { SyntheticEvent, useRef, useState } from 'react';
import ClipLoader from "react-spinners/ClipLoader";

export default function Home() {
  const [loading, setLoading] = useState(false);
  const [queryDone, setQueryDone] = useState(false);
  const [query, setQuery] = useState("");
  const [resultTitle, setResultTitle] = useState("");
  const [resMessage, setResMessage] = useState("");
  const [fields, setFields] = useState([]);
  const [columns, setColumns] = useState([[]]);
  const queryEditorRef = useRef<HTMLTextAreaElement>(null);
  const handleSubmit = (e: SyntheticEvent) => {
    e.preventDefault();

    setFields([]);
    setColumns([[]]);
    setLoading(true);
    setResultTitle("Running query...")
    fetch(process.env.NEXT_PUBLIC_API_URL+"/query" || "", {
        method : "POST",
        headers : {
            "Content-Type":"application/json; charset=utf-8"
        },
        body: JSON.stringify({"query": query})
    })
    .then(response => response.json())
    .then(data => {
      setResultTitle("Result");
      setLoading(false);
      setQueryDone(true);
      if (data.res_type == "table") {
        setFields(data.result.schema.fields);
        setColumns(data.result.columns);
      } else {
        setResMessage(JSON.stringify(data, null, 4));
      }
      queryEditorRef.current?.focus();
    });
  }

  return (
    <main>
      <TabGroup className="mt-8 max-w-screen-lg">
        <TabList>
          <Tab icon={MagnifyingGlassCircleIcon}>Query 1</Tab>
        </TabList>
        <TabPanels>
          <TabPanel>
            <Button className="mx-2" icon={PlayIcon} onClick={handleSubmit}>Run</Button>
            <Card className="mx-2 mt-2">
              <CodeEditor
                autoFocus
                id="queryEditor"
                ref={queryEditorRef}
                value={query}
                language="SQL"
                placeholder="Please enter SQL query here..."
                onChange={(evn) => setQuery(evn.target.value)}
                padding={2}
                onKeyDown={((evn) => {
                  if (evn.metaKey && evn.key === "Enter") {
                    handleSubmit(evn);
                  }
                })}
                style={{
                  fontSize: 14,
                  backgroundColor: "#FFFFFF",
                  fontFamily: 'ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace',
                }}
              />
            </Card>
            { (queryDone || loading) &&
            <Card className="mx-2 mt-2">
              <Title>{resultTitle}</Title>
            { loading && <>
              <ClipLoader color="#0066cc" loading={loading} />
              </>
            }
            { !loading && fields.length > 0 && <>
            <Table className="mt-5">
              <TableHead>
                <TableRow key="head">
                {fields.map((field, fieldIdx) => {
                  return (
                    <TableHeaderCell key={fieldIdx}>
                    { field }
                    </TableHeaderCell>
                  );
                })}
                </TableRow>
              </TableHead>
              <TableBody>
                {columns[0] && columns[0].map((_, rowIndex) => (
                    <TableRow key={rowIndex}>
                        {columns.map((col, colIdx) => {
                            return <TableCell key={colIdx}> {col[rowIndex]} </TableCell>;
                        })}
                    </TableRow>
                ))}
              </TableBody>
              </Table>
              </> || <>
              <pre><Text className="text-xs">{resMessage}</Text></pre>
            </>
            }
            </Card>
          }
          </TabPanel>
        </TabPanels>
      </TabGroup>
    </main>
  )
}
