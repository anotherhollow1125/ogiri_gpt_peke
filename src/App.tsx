import { useState, useCallback } from "react";
import PromptFieldElm from "@/components/PromptFieldElm";
import useMediaQuery from "@mui/material/useMediaQuery";
import { useMemo } from "react";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import OdaiElm from "@/components/OdaiElm";
import Grid from "@mui/material/Grid/Grid";
import { Message } from "@/structs";
import { invoke } from "@tauri-apps/api/tauri";

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

  const send = useCallback(async (odai_str: string) => {
    console.log("Process Start");

    const odai = {
      character: "user",
      content: odai_str,
    };
    setOdai(odai);

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

    console.log("Process End");
  }, []);

  return (
    <ThemeProvider theme={theme}>
      <br />
      {odai ? (
        <OdaiElm
          odai={odai}
          melchiorAnswer={melchiorAnswer}
          balthasarAnswer={balthasarAnswer}
          casperAnswer={casperAnswer}
        />
      ) : (
        <></>
      )}
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
          <PromptFieldElm send={send} />
        </Grid>
      </Grid>
    </ThemeProvider>
  );
}

export default App;
