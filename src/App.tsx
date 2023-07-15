import { useState } from "react";
import PromptFieldElm from "@/components/PromptFieldElm";
import useMediaQuery from "@mui/material/useMediaQuery";
import { useMemo } from "react";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import OdaiElm from "@/components/OdaiElm";
import Grid from "@mui/material/Grid/Grid";
import { Message } from "@/structs";

function App() {
  // https://amateur-engineer.com/react-mui-dark-mode/
  const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: prefersDarkMode ? "dark" : "light",
        },
      }),
    [prefersDarkMode]
  );

  const [odai, setOdai] = useState<Message | undefined>(undefined);

  return (
    <ThemeProvider theme={theme}>
      <br />
      {odai ? <OdaiElm odai={odai} /> : <></>}
      <Grid
        container
        sx={{
          position: "absolute",
          bottom: "0",
          width: "100%",
          padding: "20px",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <Grid
          item
          xs={12}
          sx={{ alignItems: "center", justifyContent: "center" }}
        >
          <PromptFieldElm
            send={async (odai: string) =>
              setOdai({
                character: "user",
                content: odai,
              })
            }
          />
        </Grid>
      </Grid>
    </ThemeProvider>
  );
}

export default App;

/*

// 問題なかったやつ

function App() {
  // https://amateur-engineer.com/react-mui-dark-mode/
  const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: prefersDarkMode ? "dark" : "light",
        },
      }),
    [prefersDarkMode]
  );

  const [odai, setOdai] = useState<Message | undefined>(undefined);
  const [melchiorAnswer, setMelchiorAnswer] = useState<Message | undefined>(
    undefined
  );
  const [balthasarAnswer, setBalthasarAnswer] = useState<Message | undefined>(
    undefined
  );
  const [casperAnswer, setCasperAnswer] = useState<Message | undefined>(
    undefined
  );

  useEffect(() => {
    if (odai === undefined) {
      console.log("beep");
      return;
    }

    (async () => {
      const context = [odai];

      console.log(`${context.map((m) => m.content)}`);

      const melchiorRes = await invoke<Message>("melchior", { context });

      setMelchiorAnswer(melchiorRes);
      context.push(melchiorRes);

      console.log(`${context.map((m) => m.content)}`);

      const balthasarRes = await invoke<Message>("balthasar", { context });

      setBalthasarAnswer(balthasarRes);
      context.push(balthasarRes);

      console.log(`${context.map((m) => m.content)}`);

      const casperRes = await invoke<Message>("casper", { context });

      setCasperAnswer(casperRes);

      context.push(casperRes);
      console.log(`${context.map((m) => m.content)}`);
    })();
  }, [odai]);

  const userElm = odai ? (
    <MessageElm text={odai.content} avatar={get_avatar(odai)} />
  ) : (
    <></>
  );

  const melchiorElm = melchiorAnswer ? (
    <MessageElm
      text={melchiorAnswer.content}
      avatar={get_avatar(melchiorAnswer)}
    />
  ) : (
    <></>
  );

  const balthasarElm = balthasarAnswer ? (
    <MessageElm
      text={balthasarAnswer.content}
      avatar={get_avatar(balthasarAnswer)}
    />
  ) : (
    <></>
  );

  const casperElm = casperAnswer ? (
    <MessageElm text={casperAnswer.content} avatar={get_avatar(casperAnswer)} />
  ) : (
    <></>
  );

  return (
    <ThemeProvider theme={theme}>
      <br />
      <Grid
        container
        sx={{
          width: "100%",
          alignItems: "center",
          justifyContent: "center",
          m: "0 auto",
        }}
      >
        {[userElm, melchiorElm, balthasarElm, casperElm].map((elm, id) => (
          <Grid
            item
            xs={12}
            sx={{
              alignItems: "center",
              justifyContent: "center",
            }}
            key={id}
          >
            {elm}
          </Grid>
        ))}
      </Grid>
      <Grid
        container
        sx={{
          position: "absolute",
          bottom: "0",
          width: "100%",
          padding: "20px",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <Grid
          item
          xs={12}
          sx={{ alignItems: "center", justifyContent: "center" }}
        >
          <PromptFieldElm
            send={async (odai: string) =>
              setOdai({
                character: "user",
                content: odai,
              })
            }
          />
        </Grid>
      </Grid>
    </ThemeProvider>
  );
}
*/
