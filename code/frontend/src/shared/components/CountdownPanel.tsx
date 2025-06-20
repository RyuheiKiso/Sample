// カウントダウンパネル
// 
// 親画面での設定値
// ・制限時間（秒）
// ・警告残時間（秒）
// ・警告色
// ・縦サイズ
// ・横サイズ
// ・時間のフォントサイズ
// 
// pcでの表示時
// 
// | 設定時間 | 残り時間 |
// |----------------|----------------|
// | 00:00:00 | 00:00:00 |
// 
// スマホでの表示時
// 
// | 設定時間 |
// | ---------------- |
// | 00:00:00 |
// | 残り時間 |
// | ---------------- |
// | 00:00:00 |
// 
// 親から操作できること
// ・開始
// ・停止
// ・リセット
// 
// 注意事項
// ・カウントがゼロになると停止するまでマイナスでカウントダウンする。

import React, { useEffect, useRef, useState, forwardRef, useImperativeHandle } from "react";
// MUI追加
import Box from "@mui/material/Box";
import Paper from "@mui/material/Paper";
import Table from "@mui/material/Table";
import TableHead from "@mui/material/TableHead";
import TableBody from "@mui/material/TableBody";
import TableRow from "@mui/material/TableRow";
import TableCell from "@mui/material/TableCell";
import Typography from "@mui/material/Typography";

type CountdownPanelProps = {
    limitSeconds: number; // 制限時間（秒）
    warnSeconds: number;  // 警告残時間（秒）
    warnColor: string;    // 警告色
    height: number | string; // 縦サイズ
    width: number | string;  // 横サイズ
    fontSize: number | string; // 時間のフォントサイズ
};

export type CountdownPanelHandle = {
    start: () => void;
    stop: () => void;
    reset: () => void;
};

function formatSeconds(sec: number) {
    const sign = sec < 0 ? "-" : "";
    const abs = Math.abs(sec);
    const h = Math.floor(abs / 3600);
    const m = Math.floor((abs % 3600) / 60);
    const s = abs % 60;
    return `${sign}${h.toString().padStart(2, "0")}:${m
        .toString()
        .padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
}

// 画面幅でスマホ判定するフック
function useIsMobile(breakpoint: number = 600) {
    const [isMobile, setIsMobile] = useState(
        typeof window !== "undefined" ? window.innerWidth <= breakpoint : false
    );
    useEffect(() => {
        const handler = () => setIsMobile(window.innerWidth <= breakpoint);
        window.addEventListener("resize", handler);
        return () => window.removeEventListener("resize", handler);
    }, [breakpoint]);
    return isMobile;
}

const CountdownPanel = forwardRef<CountdownPanelHandle, CountdownPanelProps>((props, ref) => {
    const {
        limitSeconds,
        warnSeconds,
        warnColor,
        height,
        width,
        fontSize,
    } = props;

    const [remaining, setRemaining] = useState(limitSeconds);
    const [running, setRunning] = useState(false);
    const timerRef = useRef<NodeJS.Timeout | null>(null);

    useImperativeHandle(ref, () => ({
        start() {
            if (!running) setRunning(true);
        },
        stop() {
            setRunning(false);
        },
        reset() {
            setRunning(false);
            setRemaining(limitSeconds);
        },
    }));

    useEffect(() => {
        setRemaining(limitSeconds);
    }, [limitSeconds]);

    useEffect(() => {
        if (!running) {
            if (timerRef.current) {
                clearInterval(timerRef.current);
                timerRef.current = null;
            }
            return;
        }
        timerRef.current = setInterval(() => {
            setRemaining((prev) => prev - 1);
        }, 1000);
        return () => {
            if (timerRef.current) {
                clearInterval(timerRef.current);
                timerRef.current = null;
            }
        };
    }, [running]);

    // 警告色判定（残り時間のみ）
    const remainingTimeColor = remaining <= warnSeconds ? warnColor : undefined;

    // 画面幅で自動判定
    const isMobile = useIsMobile();

    // スタイル
    const panelSx = {
        height: isMobile ? "auto" : height,
        minHeight: isMobile ? 0 : height,
        width: isMobile ? "100%" : width,
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        borderRadius: 2,
        boxSizing: "border-box",
        p: isMobile ? 0.5 : 1,
        bgcolor: "#fff",
        minWidth: isMobile ? 0 : undefined,
    } as const;
    const timeSx = {
        fontSize,
        fontFamily: "monospace",
        textAlign: "center",
        fontWeight: "bold",
        width: "100%",
        wordBreak: "keep-all",
        overflowWrap: "normal",
        whiteSpace: "nowrap",
        lineHeight: 1.1,
        m: 0,
    } as const;

    if (isMobile) {
        return (
            <Box width="100%" maxWidth="100vw" overflow="hidden">
                <Paper elevation={2} sx={panelSx}>
                    <Table
                        size="small"
                        sx={{
                            width: "100%",
                            minWidth: 180,
                            border: "1px solid #ccc",
                            borderCollapse: "collapse",
                            tableLayout: "fixed",
                        }}
                    >
                        <TableBody>
                            <TableRow>
                                <TableCell
                                    align="center"
                                    sx={{ border: "1px solid #ccc", background: "#f5f5f5", width: "40%" }}
                                >
                                    設定時間
                                </TableCell>
                                <TableCell
                                    align="center"
                                    sx={{ border: "1px solid #ccc", width: "60%" }}
                                >
                                    <Typography sx={timeSx}>{formatSeconds(limitSeconds)}</Typography>
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell
                                    align="center"
                                    sx={{ border: "1px solid #ccc", background: "#f5f5f5", width: "40%" }}
                                >
                                    残り時間
                                </TableCell>
                                <TableCell
                                    align="center"
                                    sx={{ border: "1px solid #ccc", width: "60%" }}
                                >
                                    <Typography sx={{ ...timeSx, color: remainingTimeColor }}>{formatSeconds(remaining)}</Typography>
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </Paper>
            </Box>
        );
    }
    return (
        <Paper elevation={2} sx={{ ...panelSx, overflowX: "auto" }}>
            <Table
                size="small"
                sx={{
                    width: "100%",
                    minWidth: 220,
                    border: "1px solid #ccc",
                    borderCollapse: "collapse",
                    tableLayout: "fixed",
                }}
            >
                <TableHead>
                    <TableRow>
                        <TableCell
                            align="center"
                            sx={{ border: "1px solid #ccc", background: "#f5f5f5" }}
                        >
                            設定時間
                        </TableCell>
                        <TableCell
                            align="center"
                            sx={{ border: "1px solid #ccc", background: "#f5f5f5" }}
                        >
                            残り時間
                        </TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <TableRow>
                        <TableCell align="center" sx={{ border: "1px solid #ccc" }}>
                            <Typography sx={timeSx}>{formatSeconds(limitSeconds)}</Typography>
                        </TableCell>
                        <TableCell align="center" sx={{ border: "1px solid #ccc" }}>
                            <Typography sx={{ ...timeSx, color: remainingTimeColor }}>{formatSeconds(remaining)}</Typography>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </Paper>
    );
});

export default CountdownPanel;
