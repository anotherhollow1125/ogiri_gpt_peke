import { useState, useEffect, useRef } from "react";
import MessageElm from "@/components/MessageElm";
import Grid from "@mui/material/Grid/Grid";
import { Message } from "@/structs";
import { get_avatar } from "@/avatar";
import { invoke } from "@tauri-apps/api/tauri";

function OdaiElm({ odai }: { odai: Message }) {
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
    (async () => {
      console.log("Process Start");

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
    })();
  }, [odai]);

  const userElm = <MessageElm text={odai.content} avatar={get_avatar(odai)} />;

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
  );
}

export default OdaiElm;
