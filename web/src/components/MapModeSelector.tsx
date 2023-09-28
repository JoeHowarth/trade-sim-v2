import { Paper, Text, Stack, Title, SegmentedControl } from "@mantine/core";

export function MapModeSelector({
  domain,
  mapMode,
  setMapMode,
}: {
  domain: [number, number];
  mapMode: string;
  setMapMode: (x: string) => void;
}) {
  return (
    <Paper
      style={{ zIndex: 3, position: 'absolute', bottom: 0, right: 0 }}
      shadow="xs"
      radius="xs"
      p="md"
      m="sm"
      withBorder
    >
      <Stack align="center">
        <Title order={4}>Map Mode</Title>
        <Text>{`${Math.round(domain[0])} to ${Math.round(domain[1])}`}</Text>
        <SegmentedControl
          orientation="vertical"
          value={mapMode}
          onChange={setMapMode}
          data={['Default', 'Price', 'Supply', 'Consumption', 'Production'].map((x) => ({
            label: x,
            value: x.toLowerCase(),
          }))}
        />
      </Stack>
    </Paper>
  );
}